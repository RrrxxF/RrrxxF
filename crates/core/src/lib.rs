//! Local domain and persistence logic shared by Tauri and cloud tests.

mod projects;
mod storage;

pub use projects::{Project, ProjectRepository};
pub use storage::CoreError;
