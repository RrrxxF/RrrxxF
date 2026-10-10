//! SQLite connection setup and ordered, transactional schema migrations.

use rusqlite::{Connection, TransactionBehavior};
use std::{fs, path::Path, time::Duration};
use thiserror::Error;

const MIGRATIONS: &[&str] = &[include_str!("../migrations/0001_projects.sql")];

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("项目名称不能为空、不能超过 80 个字符，也不能包含控制字符")]
    InvalidName,
    #[error("项目 ID 无效")]
    InvalidId,
    #[error("项目不存在或已删除")]
    NotFound,
    #[error("数据库版本 {0} 高于当前程序支持的版本")]
    FutureSchema(i64),
    #[error("无法创建本地数据目录: {0}")]
    Io(#[from] std::io::Error),
    #[error("本地数据库操作失败: {0}")]
    Database(#[from] rusqlite::Error),
}

pub(crate) fn connect(path: &Path) -> Result<Connection, CoreError> {
    let connection = Connection::open(path)?;
    connection.busy_timeout(Duration::from_secs(5))?;
    connection.pragma_update(None, "foreign_keys", "ON")?;
    Ok(connection)
}

pub(crate) fn initialize(path: &Path) -> Result<(), CoreError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut connection = connect(path)?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version > MIGRATIONS.len() as i64 {
        return Err(CoreError::FutureSchema(version));
    }

    // Each migration and its version marker commit together; reruns skip applied versions.
    for (index, migration) in MIGRATIONS.iter().enumerate().skip(version as usize) {
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute_batch(migration)?;
        transaction.pragma_update(None, "user_version", (index + 1) as i64)?;
        transaction.commit()?;
    }
    Ok(())
}
