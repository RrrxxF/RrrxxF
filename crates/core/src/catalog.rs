//! Folder and tag relationships are updated transactionally without touching original blobs.

use crate::{
    Asset, AssetRepository, CoreError,
    assets::{ensure_asset, ensure_folder, ensure_project, valid_id, valid_label},
    storage,
};
use rusqlite::{OptionalExtension, params};
use serde::Serialize;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub project_id: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub label: String,
    pub asset_count: u32,
}

impl AssetRepository {
    pub fn create_folder(
        &self,
        name: &str,
        project_id: Option<&str>,
        parent_id: Option<&str>,
    ) -> Result<Folder, CoreError> {
        let name = valid_label(name, 80)?;
        let connection = storage::connect(&self.database)?;
        if let Some(id) = project_id {
            ensure_project(&connection, id)?;
        }
        if let Some(id) = parent_id {
            ensure_folder(&connection, id, project_id)?;
        }
        let id = Uuid::new_v4().to_string();
        connection.execute("INSERT INTO folders (id, project_id, parent_id, name, created_at) VALUES (?1, ?2, ?3, ?4, strftime('%Y-%m-%dT%H:%M:%fZ','now'))", params![id, project_id, parent_id, name])?;
        Ok(Folder {
            id,
            name: name.to_string(),
            project_id: project_id.map(str::to_string),
            parent_id: parent_id.map(str::to_string),
        })
    }

    pub fn list_folders(&self, project_id: Option<&str>) -> Result<Vec<Folder>, CoreError> {
        if let Some(id) = project_id {
            valid_id(id)?;
        }
        let connection = storage::connect(&self.database)?;
        let mut statement = connection.prepare("SELECT id, name, project_id, parent_id FROM folders WHERE project_id IS ?1 ORDER BY name COLLATE NOCASE, id")?;
        statement
            .query_map([project_id], |row| {
                Ok(Folder {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    project_id: row.get(2)?,
                    parent_id: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(CoreError::from)
    }

    pub fn set_asset_folder(
        &self,
        asset_id: &str,
        folder_id: &str,
        linked: bool,
    ) -> Result<Asset, CoreError> {
        valid_id(asset_id)?;
        let connection = storage::connect(&self.database)?;
        ensure_asset(&connection, asset_id)?;
        let owner: Option<Option<String>> = connection
            .query_row(
                "SELECT project_id FROM folders WHERE id = ?1",
                [folder_id],
                |row| row.get(0),
            )
            .optional()?;
        let owner = owner.ok_or(CoreError::NotFound)?;
        if let Some(project_id) = owner {
            let linked_to_project: bool = connection.query_row("SELECT EXISTS(SELECT 1 FROM project_assets WHERE project_id = ?1 AND asset_id = ?2)", params![project_id, asset_id], |row| row.get(0))?;
            if !linked_to_project {
                return Err(CoreError::InvalidInput);
            }
        }
        if linked {
            connection.execute(
                "INSERT OR IGNORE INTO asset_folders (asset_id, folder_id) VALUES (?1, ?2)",
                params![asset_id, folder_id],
            )?;
        } else {
            connection.execute(
                "DELETE FROM asset_folders WHERE asset_id = ?1 AND folder_id = ?2",
                params![asset_id, folder_id],
            )?;
        }
        self.get(asset_id)
    }

    pub fn set_tags(&self, asset_id: &str, labels: &[String]) -> Result<Asset, CoreError> {
        valid_id(asset_id)?;
        if labels.len() > 20 {
            return Err(CoreError::InvalidInput);
        }
        let mut normalized = Vec::new();
        let mut seen = HashSet::new();
        for label in labels {
            let label = valid_label(label, 40)?;
            let key = label.to_lowercase();
            if seen.insert(key.clone()) {
                normalized.push((label.to_string(), key));
            }
        }
        let mut connection = storage::connect(&self.database)?;
        ensure_asset(&connection, asset_id)?;
        let transaction = connection.transaction()?;
        transaction.execute("DELETE FROM asset_tags WHERE asset_id = ?1", [asset_id])?;
        for (label, key) in normalized {
            transaction.execute("INSERT INTO tags (id, label, normalized_label) VALUES (?1, ?2, ?3) ON CONFLICT(normalized_label) DO NOTHING", params![Uuid::new_v4().to_string(), label, key])?;
            transaction.execute("INSERT INTO asset_tags (asset_id, tag_id) SELECT ?1, id FROM tags WHERE normalized_label = ?2", params![asset_id, key])?;
        }
        transaction.commit()?;
        self.get(asset_id)
    }

    pub fn list_tags(&self) -> Result<Vec<Tag>, CoreError> {
        let connection = storage::connect(&self.database)?;
        let mut statement = connection.prepare("SELECT t.id, t.label, COUNT(a.id) FROM tags t LEFT JOIN asset_tags at ON at.tag_id = t.id LEFT JOIN assets a ON a.id = at.asset_id AND a.deleted_at IS NULL GROUP BY t.id HAVING COUNT(a.id) > 0 ORDER BY t.label COLLATE NOCASE")?;
        statement
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    label: row.get(1)?,
                    asset_count: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(CoreError::from)
    }
}
