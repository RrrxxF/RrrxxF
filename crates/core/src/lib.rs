//! Local domain and persistence logic shared by Tauri and cloud tests.

mod assets;
mod catalog;
mod media;
mod projects;
mod storage;

pub use assets::{Asset, AssetFilter, AssetKind, AssetRepository, ImportOptions};
pub use catalog::{Folder, Tag};
pub use projects::{Project, ProjectRepository};
pub use storage::CoreError;
