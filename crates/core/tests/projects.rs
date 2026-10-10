//! Integration checks use real SQLite files and reopen them like a desktop restart.

use design_workbench_core::{CoreError, ProjectRepository};
use rusqlite::Connection;
use tempfile::tempdir;

#[test]
fn project_lifecycle_survives_reopen_and_delete_is_soft() {
    let directory = tempdir().unwrap();
    let database = directory.path().join("data/designworkbench.sqlite3");
    let repository = ProjectRepository::open(&database).unwrap();
    let alpha = repository.create("  中文项目  ").unwrap();
    let beta = repository.create("Beta").unwrap();
    repository.select(&alpha.id).unwrap();
    assert_eq!(alpha.name, "中文项目");
    assert_eq!(repository.list().unwrap().len(), 2);

    let renamed = repository.rename(&alpha.id, "新名称").unwrap();
    assert_eq!(renamed.name, "新名称");
    repository.select(&beta.id).unwrap();
    assert!(
        repository
            .set_archived(&beta.id, true)
            .unwrap()
            .archived_at
            .is_some()
    );
    assert_eq!(repository.selected_project_id().unwrap(), None);
    repository.select(&alpha.id).unwrap();
    drop(repository);

    let reopened = ProjectRepository::open(&database).unwrap();
    assert_eq!(
        reopened.selected_project_id().unwrap(),
        Some(alpha.id.clone())
    );
    let projects = reopened.list().unwrap();
    assert_eq!(projects.len(), 2);
    assert_eq!(projects[0].name, "新名称");
    assert_eq!(projects[1].id, beta.id);
    assert!(projects[1].archived_at.is_some());
    assert!(
        reopened
            .set_archived(&beta.id, false)
            .unwrap()
            .archived_at
            .is_none()
    );
    reopened.clear_selection().unwrap();
    assert_eq!(reopened.selected_project_id().unwrap(), None);
    reopened.select(&alpha.id).unwrap();
    reopened.soft_delete(&alpha.id).unwrap();
    assert_eq!(reopened.selected_project_id().unwrap(), None);
    assert_eq!(reopened.list().unwrap().len(), 1);

    let connection = Connection::open(&database).unwrap();
    let deleted_at: Option<String> = connection
        .query_row(
            "SELECT deleted_at FROM projects WHERE id = ?1",
            [&alpha.id],
            |row| row.get(0),
        )
        .unwrap();
    assert!(
        deleted_at.is_some(),
        "soft deletion must retain the database row"
    );
}

#[test]
fn rejects_bad_input_and_missing_projects() {
    let directory = tempdir().unwrap();
    let repository = ProjectRepository::open(directory.path().join("db.sqlite3")).unwrap();
    for name in ["", "  ", "bad\nname"] {
        assert!(matches!(
            repository.create(name),
            Err(CoreError::InvalidName)
        ));
    }
    assert!(matches!(
        repository.create(&"x".repeat(81)),
        Err(CoreError::InvalidName)
    ));
    assert!(matches!(
        repository.rename("not-a-uuid", "valid"),
        Err(CoreError::InvalidId)
    ));
    let missing = uuid::Uuid::new_v4().to_string();
    assert!(matches!(
        repository.rename(&missing, "valid"),
        Err(CoreError::NotFound)
    ));
    assert!(matches!(
        repository.soft_delete(&missing),
        Err(CoreError::NotFound)
    ));
    assert!(matches!(
        repository.select(&missing),
        Err(CoreError::NotFound)
    ));
}

#[test]
fn migration_is_repeatable_and_rejects_newer_schema() {
    let directory = tempdir().unwrap();
    let database = directory.path().join("db.sqlite3");
    let repository = ProjectRepository::open(&database).unwrap();
    let existing = repository.create("保留我").unwrap();
    drop(repository);

    let reopened = ProjectRepository::open(&database).unwrap();
    assert_eq!(reopened.list().unwrap()[0].id, existing.id);
    drop(reopened);
    let connection = Connection::open(&database).unwrap();
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap();
    assert_eq!(version, 2);
    connection.pragma_update(None, "user_version", 99).unwrap();
    drop(connection);
    assert!(matches!(
        ProjectRepository::open(&database),
        Err(CoreError::FutureSchema(99))
    ));
    let connection = Connection::open(&database).unwrap();
    let count: i64 = connection
        .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1, "an unsupported schema must not be destroyed");
}
