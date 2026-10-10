//! Project rules and SQL operations. UI and Tauri commands never issue raw SQL.

use crate::storage::{self, CoreError};
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: Option<String>,
    pub archived_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ProjectRepository {
    path: PathBuf,
}

impl ProjectRepository {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let path = path.into();
        storage::initialize(&path)?;
        Ok(Self { path })
    }

    pub fn database_path(&self) -> &Path {
        &self.path
    }

    pub fn selected_project_id(&self) -> Result<Option<String>, CoreError> {
        let connection = storage::connect(&self.path)?;
        connection
            .query_row(
                "SELECT value FROM app_settings WHERE key = 'selected_project_id'",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(CoreError::from)
    }

    pub fn select(&self, id: &str) -> Result<(), CoreError> {
        let id = valid_id(id)?;
        let connection = storage::connect(&self.path)?;
        get(&connection, id)?;
        connection.execute(
            "INSERT INTO app_settings (key, value) VALUES ('selected_project_id', ?1) \
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [id],
        )?;
        Ok(())
    }

    pub fn clear_selection(&self) -> Result<(), CoreError> {
        let connection = storage::connect(&self.path)?;
        connection.execute(
            "DELETE FROM app_settings WHERE key = 'selected_project_id'",
            [],
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Project>, CoreError> {
        let connection = storage::connect(&self.path)?;
        let mut statement = connection.prepare(
            "SELECT id, name, description, color, archived_at, created_at, updated_at \
             FROM projects WHERE deleted_at IS NULL \
             ORDER BY archived_at IS NOT NULL, name COLLATE NOCASE, created_at, id",
        )?;
        let rows = statement.query_map([], project_from_row)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(CoreError::from)
    }

    pub fn create(&self, name: &str) -> Result<Project, CoreError> {
        let name = valid_name(name)?;
        let id = Uuid::new_v4().to_string();
        let connection = storage::connect(&self.path)?;
        connection.execute(
            "INSERT INTO projects (id, name, created_at, updated_at) \
             VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ','now'), strftime('%Y-%m-%dT%H:%M:%fZ','now'))",
            params![id, name],
        )?;
        get(&connection, &id)
    }

    pub fn rename(&self, id: &str, name: &str) -> Result<Project, CoreError> {
        let id = valid_id(id)?;
        let name = valid_name(name)?;
        let connection = storage::connect(&self.path)?;
        let changed = connection.execute(
            "UPDATE projects SET name = ?2, updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now') \
             WHERE id = ?1 AND deleted_at IS NULL",
            params![id, name],
        )?;
        if changed == 0 {
            return Err(CoreError::NotFound);
        }
        get(&connection, id)
    }

    pub fn set_archived(&self, id: &str, archived: bool) -> Result<Project, CoreError> {
        let id = valid_id(id)?;
        let mut connection = storage::connect(&self.path)?;
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let changed = transaction.execute(
            "UPDATE projects SET archived_at = CASE WHEN ?2 THEN strftime('%Y-%m-%dT%H:%M:%fZ','now') ELSE NULL END, \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id = ?1 AND deleted_at IS NULL",
            params![id, archived],
        )?;
        if changed == 0 {
            return Err(CoreError::NotFound);
        }
        if archived {
            transaction.execute(
                "DELETE FROM app_settings WHERE key = 'selected_project_id' AND value = ?1",
                [id],
            )?;
        }
        transaction.commit()?;
        get(&connection, id)
    }

    pub fn soft_delete(&self, id: &str) -> Result<(), CoreError> {
        let id = valid_id(id)?;
        let mut connection = storage::connect(&self.path)?;
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let changed = transaction.execute(
            "UPDATE projects SET deleted_at = strftime('%Y-%m-%dT%H:%M:%fZ','now'), \
             updated_at = strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id = ?1 AND deleted_at IS NULL",
            params![id],
        )?;
        if changed == 0 {
            return Err(CoreError::NotFound);
        }
        transaction.execute(
            "DELETE FROM app_settings WHERE key = 'selected_project_id' AND value = ?1",
            [id],
        )?;
        transaction.commit()?;
        Ok(())
    }
}

fn valid_name(name: &str) -> Result<&str, CoreError> {
    let name = name.trim();
    if name.is_empty() || name.chars().count() > 80 || name.chars().any(char::is_control) {
        return Err(CoreError::InvalidName);
    }
    Ok(name)
}

fn valid_id(id: &str) -> Result<&str, CoreError> {
    Uuid::parse_str(id).map_err(|_| CoreError::InvalidId)?;
    Ok(id)
}

fn project_from_row(row: &Row<'_>) -> rusqlite::Result<Project> {
    Ok(Project {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        color: row.get(3)?,
        archived_at: row.get(4)?,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

fn get(connection: &Connection, id: &str) -> Result<Project, CoreError> {
    connection
        .query_row(
            "SELECT id, name, description, color, archived_at, created_at, updated_at \
             FROM projects WHERE id = ?1 AND deleted_at IS NULL",
            [id],
            project_from_row,
        )
        .optional()?
        .ok_or(CoreError::NotFound)
}
