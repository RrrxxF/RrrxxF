//! Native application bootstrap and private per-user database placement.

mod commands;

use design_workbench_core::ProjectRepository;
use std::path::PathBuf;
use tauri::Manager;

pub(crate) struct AppState {
    projects: ProjectRepository,
    data_directory: PathBuf,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_directory = app.path().app_data_dir()?;
            let database = data_directory.join("data").join("designworkbench.sqlite3");
            let projects = ProjectRepository::open(database)?;
            app.manage(AppState {
                projects,
                data_directory,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
