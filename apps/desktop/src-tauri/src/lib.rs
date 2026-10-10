//! Native application bootstrap and private per-user database placement.

mod asset_commands;
mod commands;

use design_workbench_core::{AssetRepository, ProjectRepository};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, atomic::AtomicBool},
};
use tauri::Manager;

pub(crate) struct AppState {
    projects: ProjectRepository,
    assets: AssetRepository,
    data_directory: PathBuf,
    cancelled_imports: Mutex<HashMap<String, Arc<AtomicBool>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_directory = app.path().app_data_dir()?;
            let database = data_directory.join("data").join("designworkbench.sqlite3");
            let projects = ProjectRepository::open(database)?;
            let assets = AssetRepository::open(&data_directory)?;
            app.manage(AppState {
                projects,
                assets,
                data_directory,
                cancelled_imports: Mutex::new(HashMap::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::list_projects,
            commands::get_selected_project_id,
            commands::select_project,
            commands::clear_project_selection,
            commands::create_project,
            commands::rename_project,
            commands::set_project_archived,
            commands::delete_project,
            asset_commands::list_assets,
            asset_commands::import_asset,
            asset_commands::cancel_import,
            asset_commands::finish_import,
            asset_commands::get_asset_thumbnail,
            asset_commands::rename_asset,
            asset_commands::delete_asset,
            asset_commands::set_asset_favorite,
            asset_commands::link_asset_project,
            asset_commands::set_asset_tags,
            asset_commands::create_folder,
            asset_commands::list_folders,
            asset_commands::set_asset_folder,
            asset_commands::list_tags,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
