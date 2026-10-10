//! Small IPC boundary; project validation and persistence live in the core crate.

use crate::AppState;
use design_workbench_core::Project;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppInfo {
    version: &'static str,
    data_directory: String,
}

#[tauri::command]
pub(crate) fn get_app_info(state: State<'_, AppState>) -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION"),
        data_directory: state.data_directory.to_string_lossy().into_owned(),
    }
}

#[tauri::command]
pub(crate) fn list_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state.projects.list().map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn get_selected_project_id(
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    state
        .projects
        .selected_project_id()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn select_project(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .projects
        .select(&id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn clear_project_selection(state: State<'_, AppState>) -> Result<(), String> {
    state
        .projects
        .clear_selection()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn create_project(state: State<'_, AppState>, name: String) -> Result<Project, String> {
    state
        .projects
        .create(&name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn rename_project(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> Result<Project, String> {
    state
        .projects
        .rename(&id, &name)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn set_project_archived(
    state: State<'_, AppState>,
    id: String,
    archived: bool,
) -> Result<Project, String> {
    state
        .projects
        .set_archived(&id, archived)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn delete_project(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state
        .projects
        .soft_delete(&id)
        .map_err(|error| error.to_string())
}
