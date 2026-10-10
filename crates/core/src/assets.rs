//! Asset metadata, project links and search; all writes reference content-addressed blobs.

use crate::{CoreError, media, storage};
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    Icon,
    Component,
    Image,
}

impl AssetKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Icon => "icon",
            Self::Component => "component",
            Self::Image => "image",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportOptions {
    pub kind: AssetKind,
    pub project_id: Option<String>,
    pub folder_id: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetFilter {
    pub kind: Option<AssetKind>,
    pub project_id: Option<String>,
    pub folder_id: Option<String>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub favorites_only: bool,
    pub offset: u32,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub kind: AssetKind,
    pub original_filename: String,
    pub mime: String,
    pub byte_size: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub sha256: String,
    pub has_thumbnail: bool,
    pub favorite: bool,
    pub project_ids: Vec<String>,
    pub folder_ids: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: String,
}

#[derive(Clone)]
pub struct AssetRepository {
    pub(crate) database: PathBuf,
    root: PathBuf,
}

impl AssetRepository {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let root = root.into();
        let database = root.join("data/designworkbench.sqlite3");
        storage::initialize(&database)?;
        fs::create_dir_all(root.join("import-staging"))?;
        Ok(Self { database, root })
    }

    pub fn import_file(
        &self,
        source: &Path,
        options: &ImportOptions,
        cancelled: impl Fn() -> bool,
    ) -> Result<Asset, CoreError> {
        self.import_file_with_available(source, options, cancelled, None)
    }

    /// The optional capacity models a full disk in integration tests; production reads actual free space.
    pub fn import_file_with_available(
        &self,
        source: &Path,
        options: &ImportOptions,
        cancelled: impl Fn() -> bool,
        available_override: Option<u64>,
    ) -> Result<Asset, CoreError> {
        let original_filename = source
            .file_name()
            .ok_or(CoreError::InvalidInput)?
            .to_string_lossy()
            .into_owned();
        let proposed_name = source
            .file_stem()
            .ok_or(CoreError::InvalidInput)?
            .to_string_lossy();
        let name: String = proposed_name.trim().chars().take(120).collect();
        valid_label(&name, 120)?;
        let connection = storage::connect(&self.database)?;
        if let Some(id) = &options.project_id {
            ensure_project(&connection, id)?;
        }
        if let Some(id) = &options.folder_id {
            ensure_folder(&connection, id, options.project_id.as_deref())?;
        }
        drop(connection);

        let staged = media::stage(source, &self.root, &cancelled, available_override)?;
        if cancelled() {
            return Err(CoreError::Cancelled);
        }
        let (blob_path, new_blob, thumbnail_path) = media::commit_file(&staged, &self.root)?;
        if cancelled() {
            if new_blob {
                let _ = fs::remove_file(blob_path);
            }
            return Err(CoreError::Cancelled);
        }
        let result = self.insert_asset(
            &staged,
            &name,
            &original_filename,
            options,
            thumbnail_path.is_some(),
        );
        if result.is_err() && new_blob {
            // A failed transaction must not leave a newly linked original behind.
            let connection = storage::connect(&self.database)?;
            let references: i64 = connection.query_row(
                "SELECT COUNT(*) FROM blobs WHERE sha256 = ?1",
                [&staged.sha256],
                |row| row.get(0),
            )?;
            if references == 0 {
                let _ = fs::remove_file(blob_path);
            }
        }
        result
    }

    fn insert_asset(
        &self,
        staged: &media::StagedMedia,
        name: &str,
        original_filename: &str,
        options: &ImportOptions,
        has_thumbnail: bool,
    ) -> Result<Asset, CoreError> {
        let id = Uuid::new_v4().to_string();
        let mut connection = storage::connect(&self.database)?;
        let transaction = connection.transaction()?;
        let rel_path = format!("media/blobs/{}/{}", &staged.sha256[..2], staged.sha256);
        let thumb_rel = has_thumbnail.then(|| format!("cache/thumbs/{}_320.webp", staged.sha256));
        transaction.execute(
            "INSERT OR IGNORE INTO blobs (sha256, rel_path, mime, byte_size, thumbnail_rel_path, width, height, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, strftime('%Y-%m-%dT%H:%M:%fZ','now'))",
            params![staged.sha256, rel_path, staged.mime, staged.byte_size, thumb_rel, staged.dimensions.map(|d| d.0), staged.dimensions.map(|d| d.1)],
        )?;
        transaction.execute(
            "INSERT INTO assets (id, blob_sha256, kind, name, original_filename, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, strftime('%Y-%m-%dT%H:%M:%fZ','now'), strftime('%Y-%m-%dT%H:%M:%fZ','now'))",
            params![id, staged.sha256, options.kind.as_str(), name, original_filename],
        )?;
        if let Some(project_id) = &options.project_id {
            transaction.execute(
                "INSERT INTO project_assets (project_id, asset_id) VALUES (?1, ?2)",
                params![project_id, id],
            )?;
        }
        if let Some(folder_id) = &options.folder_id {
            transaction.execute(
                "INSERT INTO asset_folders (asset_id, folder_id) VALUES (?1, ?2)",
                params![id, folder_id],
            )?;
        }
        transaction.commit()?;
        self.get(&id)
    }

    pub fn get(&self, id: &str) -> Result<Asset, CoreError> {
        valid_id(id)?;
        let connection = storage::connect(&self.database)?;
        load_asset(&connection, id)
    }

    pub fn list(&self, filter: &AssetFilter) -> Result<Vec<Asset>, CoreError> {
        let connection = storage::connect(&self.database)?;
        if let Some(id) = &filter.project_id {
            valid_id(id)?;
        }
        if let Some(id) = &filter.folder_id {
            valid_id(id)?;
        }
        if filter
            .search
            .as_deref()
            .is_some_and(|value| value.chars().count() > 120)
            || filter
                .tag
                .as_deref()
                .is_some_and(|value| value.chars().count() > 40)
        {
            return Err(CoreError::InvalidInput);
        }
        let search = filter
            .search
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(like_pattern);
        let tag = filter
            .tag
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .map(|value| value.trim().to_lowercase());
        let mut statement = connection.prepare(
            "SELECT a.id FROM assets a WHERE a.deleted_at IS NULL \
             AND (?1 IS NULL OR a.kind = ?1) \
             AND (?2 IS NULL OR EXISTS (SELECT 1 FROM project_assets pa WHERE pa.asset_id = a.id AND pa.project_id = ?2)) \
             AND (?3 IS NULL OR EXISTS (SELECT 1 FROM asset_folders af WHERE af.asset_id = a.id AND af.folder_id = ?3)) \
             AND (?4 IS NULL OR EXISTS (SELECT 1 FROM asset_tags at JOIN tags t ON t.id = at.tag_id WHERE at.asset_id = a.id AND t.normalized_label = ?4)) \
             AND (?5 = 0 OR EXISTS (SELECT 1 FROM favorites f WHERE f.asset_id = a.id)) \
             AND (?6 IS NULL OR a.name LIKE ?6 ESCAPE '\\' OR a.original_filename LIKE ?6 ESCAPE '\\' \
               OR EXISTS (SELECT 1 FROM asset_tags at JOIN tags t ON t.id = at.tag_id WHERE at.asset_id = a.id AND t.label LIKE ?6 ESCAPE '\\')) \
             ORDER BY a.created_at DESC, a.id DESC LIMIT 50 OFFSET ?7",
        )?;
        let ids = statement
            .query_map(
                params![
                    filter.kind.map(AssetKind::as_str),
                    filter.project_id,
                    filter.folder_id,
                    tag,
                    filter.favorites_only,
                    search,
                    filter.offset
                ],
                |row| row.get::<_, String>(0),
            )?
            .collect::<Result<Vec<_>, _>>()?;
        ids.iter().map(|id| load_asset(&connection, id)).collect()
    }

    pub fn rename(&self, id: &str, name: &str) -> Result<Asset, CoreError> {
        valid_id(id)?;
        let name = valid_label(name, 120)?;
        let connection = storage::connect(&self.database)?;
        let changed = connection.execute("UPDATE assets SET name = ?2, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id = ?1 AND deleted_at IS NULL", params![id, name])?;
        if changed == 0 {
            return Err(CoreError::NotFound);
        }
        load_asset(&connection, id)
    }

    pub fn set_favorite(&self, id: &str, favorite: bool) -> Result<Asset, CoreError> {
        valid_id(id)?;
        let connection = storage::connect(&self.database)?;
        ensure_asset(&connection, id)?;
        if favorite {
            connection.execute("INSERT OR IGNORE INTO favorites (asset_id, created_at) VALUES (?1, strftime('%Y-%m-%dT%H:%M:%fZ','now'))", [id])?;
        } else {
            connection.execute("DELETE FROM favorites WHERE asset_id = ?1", [id])?;
        }
        load_asset(&connection, id)
    }

    pub fn link_project(
        &self,
        asset_id: &str,
        project_id: &str,
        linked: bool,
    ) -> Result<Asset, CoreError> {
        valid_id(asset_id)?;
        let mut connection = storage::connect(&self.database)?;
        ensure_asset(&connection, asset_id)?;
        if linked {
            ensure_project(&connection, project_id)?;
        } else {
            valid_id(project_id)?;
        }
        let transaction = connection.transaction()?;
        if linked {
            transaction.execute(
                "INSERT OR IGNORE INTO project_assets (project_id, asset_id) VALUES (?1, ?2)",
                params![project_id, asset_id],
            )?;
        } else {
            transaction.execute(
                "DELETE FROM project_assets WHERE project_id = ?1 AND asset_id = ?2",
                params![project_id, asset_id],
            )?;
            transaction.execute(
                "DELETE FROM asset_folders WHERE asset_id = ?1 AND folder_id IN (SELECT id FROM folders WHERE project_id = ?2)",
                params![asset_id, project_id],
            )?;
        }
        transaction.commit()?;
        load_asset(&connection, asset_id)
    }

    pub fn soft_delete(&self, id: &str) -> Result<(), CoreError> {
        valid_id(id)?;
        let connection = storage::connect(&self.database)?;
        let changed = connection.execute("UPDATE assets SET deleted_at = strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id = ?1 AND deleted_at IS NULL", [id])?;
        if changed == 0 {
            return Err(CoreError::NotFound);
        }
        Ok(())
    }

    pub fn thumbnail(&self, id: &str) -> Result<Option<Vec<u8>>, CoreError> {
        valid_id(id)?;
        let connection = storage::connect(&self.database)?;
        let sha: Option<String> = connection.query_row(
            "SELECT b.sha256 FROM assets a JOIN blobs b ON b.sha256 = a.blob_sha256 WHERE a.id = ?1 AND a.deleted_at IS NULL AND b.thumbnail_rel_path IS NOT NULL", [id], |row| row.get(0)
        ).optional()?;
        sha.map(|hash| {
            if hash.len() != 64 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err(CoreError::InvalidInput);
            }
            fs::read(media::thumbnail_path(&self.root, &hash)).map_err(CoreError::from)
        })
        .transpose()
    }

    pub fn data_root(&self) -> &Path {
        &self.root
    }
}

pub(crate) fn valid_id(id: &str) -> Result<(), CoreError> {
    Uuid::parse_str(id).map_err(|_| CoreError::InvalidId)?;
    Ok(())
}

pub(crate) fn valid_label(value: &str, max: usize) -> Result<&str, CoreError> {
    let value = value.trim();
    if value.is_empty() || value.chars().count() > max || value.chars().any(char::is_control) {
        return Err(CoreError::InvalidInput);
    }
    Ok(value)
}

pub(crate) fn ensure_project(connection: &Connection, id: &str) -> Result<(), CoreError> {
    valid_id(id)?;
    let exists: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1 AND deleted_at IS NULL AND archived_at IS NULL)", [id], |row| row.get(0))?;
    if exists {
        Ok(())
    } else {
        Err(CoreError::NotFound)
    }
}

pub(crate) fn ensure_folder(
    connection: &Connection,
    id: &str,
    project_id: Option<&str>,
) -> Result<(), CoreError> {
    valid_id(id)?;
    let owner: Option<Option<String>> = connection
        .query_row(
            "SELECT project_id FROM folders WHERE id = ?1",
            [id],
            |row| row.get(0),
        )
        .optional()?;
    match owner {
        Some(owner) if owner.as_deref() == project_id => Ok(()),
        _ => Err(CoreError::NotFound),
    }
}

pub(crate) fn ensure_asset(connection: &Connection, id: &str) -> Result<(), CoreError> {
    let exists: bool = connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM assets WHERE id = ?1 AND deleted_at IS NULL)",
        [id],
        |row| row.get(0),
    )?;
    if exists {
        Ok(())
    } else {
        Err(CoreError::NotFound)
    }
}

fn load_asset(connection: &Connection, id: &str) -> Result<Asset, CoreError> {
    let mut asset = connection.query_row(
        "SELECT a.id, a.name, a.kind, a.original_filename, b.mime, b.byte_size, b.width, b.height, b.sha256, \
         b.thumbnail_rel_path IS NOT NULL, EXISTS(SELECT 1 FROM favorites f WHERE f.asset_id = a.id), a.created_at \
         FROM assets a JOIN blobs b ON b.sha256 = a.blob_sha256 WHERE a.id = ?1 AND a.deleted_at IS NULL", [id], asset_from_row
    ).optional()?.ok_or(CoreError::NotFound)?;
    asset.project_ids = list_strings(
        connection,
        "SELECT pa.project_id FROM project_assets pa JOIN projects p ON p.id = pa.project_id WHERE pa.asset_id = ?1 AND p.deleted_at IS NULL",
        id,
    )?;
    asset.folder_ids = list_strings(
        connection,
        "SELECT folder_id FROM asset_folders WHERE asset_id = ?1",
        id,
    )?;
    asset.tags = list_strings(
        connection,
        "SELECT t.label FROM tags t JOIN asset_tags at ON at.tag_id = t.id WHERE at.asset_id = ?1 ORDER BY t.label",
        id,
    )?;
    Ok(asset)
}

fn asset_from_row(row: &Row<'_>) -> rusqlite::Result<Asset> {
    let kind: String = row.get(2)?;
    let kind = match kind.as_str() {
        "icon" => AssetKind::Icon,
        "component" => AssetKind::Component,
        _ => AssetKind::Image,
    };
    Ok(Asset {
        id: row.get(0)?,
        name: row.get(1)?,
        kind,
        original_filename: row.get(3)?,
        mime: row.get(4)?,
        byte_size: row.get(5)?,
        width: row.get(6)?,
        height: row.get(7)?,
        sha256: row.get(8)?,
        has_thumbnail: row.get(9)?,
        favorite: row.get(10)?,
        project_ids: Vec::new(),
        folder_ids: Vec::new(),
        tags: Vec::new(),
        created_at: row.get(11)?,
    })
}

fn list_strings(connection: &Connection, sql: &str, id: &str) -> Result<Vec<String>, CoreError> {
    let mut statement = connection.prepare(sql)?;
    statement
        .query_map([id], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(CoreError::from)
}

fn like_pattern(value: &str) -> String {
    let escaped = value
        .trim()
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    format!("%{escaped}%")
}
