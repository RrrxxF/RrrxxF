//! S2 integration checks exercise real files, SQLite relations and restart-safe blob reuse.

use design_workbench_core::{
    AssetFilter, AssetKind, AssetRepository, CoreError, ImportOptions, ProjectRepository,
};
use image::{ImageBuffer, Rgba};
use rusqlite::Connection;
use std::{fs, path::Path};
use tempfile::tempdir;

fn options(kind: AssetKind, project_id: Option<String>) -> ImportOptions {
    ImportOptions {
        kind,
        project_id,
        folder_id: None,
    }
}

fn png(path: &Path) {
    let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(4, 4, Rgba([120, 40, 220, 255]));
    image.save(path).unwrap();
}

#[test]
fn duplicate_content_and_shared_project_links_survive_restart_and_project_delete() {
    let directory = tempdir().unwrap();
    let root = directory.path().join("app");
    let projects = ProjectRepository::open(root.join("data/designworkbench.sqlite3")).unwrap();
    let alpha = projects.create("甲项目").unwrap();
    let beta = projects.create("乙项目").unwrap();
    let source = directory.path().join("中文 图标.png");
    png(&source);
    let assets = AssetRepository::open(&root).unwrap();
    let first = assets
        .import_file(
            &source,
            &options(AssetKind::Icon, Some(alpha.id.clone())),
            || false,
        )
        .unwrap();
    let second = assets
        .import_file(
            &source,
            &options(AssetKind::Icon, Some(beta.id.clone())),
            || false,
        )
        .unwrap();
    assert_ne!(first.id, second.id);
    assert_eq!(first.sha256, second.sha256);
    assert_eq!(first.name, "中文 图标");
    assert!(first.has_thumbnail);
    assert!(assets.thumbnail(&first.id).unwrap().unwrap().len() > 20);
    drop(assets);

    let reopened = AssetRepository::open(&root).unwrap();
    assert_eq!(
        reopened
            .list(&AssetFilter {
                project_id: Some(beta.id.clone()),
                ..Default::default()
            })
            .unwrap()
            .len(),
        1
    );
    projects.soft_delete(&alpha.id).unwrap();
    assert_eq!(reopened.get(&second.id).unwrap().sha256, first.sha256);
    let connection = Connection::open(root.join("data/designworkbench.sqlite3")).unwrap();
    let blobs: i64 = connection
        .query_row("SELECT COUNT(*) FROM blobs", [], |row| row.get(0))
        .unwrap();
    assert_eq!(blobs, 1);
    assert!(
        root.join(format!(
            "media/blobs/{}/{}",
            &first.sha256[..2],
            first.sha256
        ))
        .exists()
    );
    reopened.soft_delete(&first.id).unwrap();
    assert!(
        root.join(format!(
            "media/blobs/{}/{}",
            &first.sha256[..2],
            first.sha256
        ))
        .exists()
    );
}

#[test]
fn one_svg_asset_can_leave_project_a_without_affecting_project_b() {
    let directory = tempdir().unwrap();
    let root = directory.path().join("app");
    let projects = ProjectRepository::open(root.join("data/designworkbench.sqlite3")).unwrap();
    let alpha = projects.create("A").unwrap();
    let beta = projects.create("B").unwrap();
    let source = directory.path().join("共享.svg");
    fs::write(
        &source,
        "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0L1 1\"/></svg>",
    )
    .unwrap();
    let assets = AssetRepository::open(&root).unwrap();
    let asset = assets
        .import_file(
            &source,
            &options(AssetKind::Icon, Some(alpha.id.clone())),
            || false,
        )
        .unwrap();
    assert_eq!(asset.mime, "image/svg+xml");
    assert!(!asset.has_thumbnail);
    assets.link_project(&asset.id, &beta.id, true).unwrap();
    let remaining = assets.link_project(&asset.id, &alpha.id, false).unwrap();
    assert_eq!(remaining.project_ids, vec![beta.id.clone()]);
    assert_eq!(
        assets
            .list(&AssetFilter {
                project_id: Some(beta.id),
                ..Default::default()
            })
            .unwrap()
            .len(),
        1
    );
    assert!(
        root.join(format!(
            "media/blobs/{}/{}",
            &asset.sha256[..2],
            asset.sha256
        ))
        .exists()
    );
}

#[test]
fn chinese_search_tags_folders_favorites_and_200_file_batch() {
    let directory = tempdir().unwrap();
    let root = directory.path().join("app");
    let projects = ProjectRepository::open(root.join("data/designworkbench.sqlite3")).unwrap();
    let project = projects.create("移动端").unwrap();
    let assets = AssetRepository::open(&root).unwrap();
    let folder = assets
        .create_folder("首页", Some(&project.id), None)
        .unwrap();
    let source_dir = directory.path().join("sources");
    fs::create_dir(&source_dir).unwrap();
    let mut chosen_id = String::new();
    for index in 0..200 {
        let source = source_dir.join(format!("按钮图标{index}.png"));
        png(&source);
        let imported = assets
            .import_file(
                &source,
                &options(AssetKind::Image, Some(project.id.clone())),
                || false,
            )
            .unwrap();
        if index == 42 {
            chosen_id = imported.id;
        }
    }
    let tagged = assets
        .set_tags(&chosen_id, &["中文标签".into(), "UI".into(), "ui".into()])
        .unwrap();
    assert_eq!(tagged.tags.len(), 2);
    assets
        .set_asset_folder(&chosen_id, &folder.id, true)
        .unwrap();
    assets.set_favorite(&chosen_id, true).unwrap();
    let filtered = assets
        .list(&AssetFilter {
            kind: Some(AssetKind::Image),
            project_id: Some(project.id.clone()),
            folder_id: Some(folder.id.clone()),
            tag: Some("中文标签".into()),
            search: Some("按钮".into()),
            favorites_only: true,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id, chosen_id);
    drop(assets);
    let assets = AssetRepository::open(&root).unwrap();
    assert_eq!(assets.get(&chosen_id).unwrap().tags.len(), 2);
    assert!(assets.get(&chosen_id).unwrap().favorite);
    assert!(
        assets
            .get(&chosen_id)
            .unwrap()
            .folder_ids
            .contains(&folder.id)
    );
    assert_eq!(
        assets
            .list(&AssetFilter {
                offset: 150,
                ..Default::default()
            })
            .unwrap()
            .len(),
        50
    );
    assert_eq!(
        assets
            .list_tags()
            .unwrap()
            .iter()
            .find(|tag| tag.label == "中文标签")
            .unwrap()
            .asset_count,
        1
    );
}

#[test]
fn invalid_format_cancellation_and_simulated_full_disk_leave_no_records() {
    let directory = tempdir().unwrap();
    let root = directory.path().join("app");
    let assets = AssetRepository::open(&root).unwrap();
    let source = directory.path().join("forged.png");
    fs::write(&source, b"not an image").unwrap();
    assert!(matches!(
        assets.import_file(&source, &options(AssetKind::Image, None), || false),
        Err(CoreError::UnsupportedFormat)
    ));
    png(&source);
    assert!(matches!(
        assets.import_file(&source, &options(AssetKind::Image, None), || true),
        Err(CoreError::Cancelled)
    ));
    assert!(matches!(
        assets.import_file_with_available(
            &source,
            &options(AssetKind::Image, None),
            || false,
            Some(1)
        ),
        Err(CoreError::InsufficientSpace)
    ));
    assert!(assets.list(&AssetFilter::default()).unwrap().is_empty());
    assert_eq!(
        fs::read_dir(root.join("import-staging")).unwrap().count(),
        0
    );
}

#[test]
fn upgrade_from_s1_preserves_projects_and_creates_snapshot() {
    let directory = tempdir().unwrap();
    let root = directory.path().join("app");
    fs::create_dir_all(root.join("data")).unwrap();
    let database = root.join("data/designworkbench.sqlite3");
    let connection = Connection::open(&database).unwrap();
    connection
        .execute_batch(include_str!("../migrations/0001_projects.sql"))
        .unwrap();
    connection.pragma_update(None, "user_version", 1).unwrap();
    connection.execute("INSERT INTO projects (id, name, created_at, updated_at) VALUES (?1, '旧项目', '2026', '2026')", [uuid::Uuid::new_v4().to_string()]).unwrap();
    drop(connection);
    let assets = AssetRepository::open(&root).unwrap();
    assert!(assets.list(&AssetFilter::default()).unwrap().is_empty());
    assert_eq!(
        ProjectRepository::open(&database).unwrap().list().unwrap()[0].name,
        "旧项目"
    );
    assert_eq!(fs::read_dir(root.join("backups")).unwrap().count(), 1);
}
