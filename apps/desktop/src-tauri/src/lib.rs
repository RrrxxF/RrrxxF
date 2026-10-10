// S0 native entry point: expose only version information until S1 adds local data services.
#[tauri::command]
fn app_info() -> String {
    format!("DesignWorkbench {}", env!("CARGO_PKG_VERSION"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
