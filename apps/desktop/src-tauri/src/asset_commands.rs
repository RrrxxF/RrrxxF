//! Native asset IPC delegates file validation and SQL changes to the core service.

use crate::AppState;
use design_workbench_core::{Asset, AssetFilter, Folder, ImportOptions, Tag};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub(crate) fn list_assets(
    state: State<'_, AppState>,
    filter: AssetFilter,
) -> Result<Vec<Asset>, String> {
    state
        .assets
        .list(&filter)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) async fn import_asset(
    state: State<'_, AppState>,
    path: String,
    options: ImportOptions,
    job_id: String,
) -> Result<Asset, String> {
    Uuid::parse_str(&job_id).map_err(|_| "导入任务 ID 无效".to_string())?;
    let cancelled = {
        let mut jobs = state
            .cancelled_imports
            .lock()
            .map_err(|_| "导入状态不可用".to_string())?;
        Arc::clone(
            jobs.entry(job_id)
                .or_insert_with(|| Arc::new(AtomicBool::new(false))),
        )
    };
    let repository = state.assets.clone();
    tauri::async_runtime::spawn_blocking(move || {
        repository.import_file(&PathBuf::from(path), &options, || {
            cancelled.load(Ordering::Relaxed)
        })
    })
    .await
    .map_err(|error| error.to_string())?
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn cancel_import(state: State<'_, AppState>, job_id: String) -> Result<(), String> {
    Uuid::parse_str(&job_id).map_err(|_| "导入任务 ID 无效".to_string())?;
    let mut jobs = state
        .cancelled_imports
        .lock()
        .map_err(|_| "导入状态不可用".to_string())?;
    jobs.entry(job_id)
        .or_insert_with(|| Arc::new(AtomicBool::new(true)))
        .store(true, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub(crate) fn finish_import(state: State<'_, AppState>, job_id: String) -> Result<(), String> {
    state
        .cancelled_imports
        .lock()
        .map_err(|_| "导入状态不可用".to_string())?
        .remove(&job_id);
    Ok(())
}

#[tauri::command]
pub(crate) fn get_asset_thumbnail(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<Vec<u8>>, String> {
    state
        .assets
        .thumbnail(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn rename_asset(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> Result<Asset, String> {
    state
        .assets
        .rename(&id, &name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn delete_asset(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .assets
        .soft_delete(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn set_asset_favorite(
    state: State<'_, AppState>,
    id: String,
    favorite: bool,
) -> Result<Asset, String> {
    state
        .assets
        .set_favorite(&id, favorite)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn link_asset_project(
    state: State<'_, AppState>,
    asset_id: String,
    project_id: String,
    linked: bool,
) -> Result<Asset, String> {
    state
        .assets
        .link_project(&asset_id, &project_id, linked)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn set_asset_tags(
    state: State<'_, AppState>,
    asset_id: String,
    labels: Vec<String>,
) -> Result<Asset, String> {
    state
        .assets
        .set_tags(&asset_id, &labels)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn create_folder(
    state: State<'_, AppState>,
    name: String,
    project_id: Option<String>,
    parent_id: Option<String>,
) -> Result<Folder, String> {
    state
        .assets
        .create_folder(&name, project_id.as_deref(), parent_id.as_deref())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn list_folders(
    state: State<'_, AppState>,
    project_id: Option<String>,
) -> Result<Vec<Folder>, String> {
    state
        .assets
        .list_folders(project_id.as_deref())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn set_asset_folder(
    state: State<'_, AppState>,
    asset_id: String,
    folder_id: String,
    linked: bool,
) -> Result<Asset, String> {
    state
        .assets
        .set_asset_folder(&asset_id, &folder_id, linked)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn list_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    state.assets.list_tags().map_err(|error| error.to_string())
}
