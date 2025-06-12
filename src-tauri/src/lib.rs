use tauri::Manager;

mod api;
mod fs;
mod settings;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();

            let settings_manager = settings::models::AppSettingsManager::new(&handle) ;
            app.manage(settings_manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fs::api::get_drives,
            settings::api::get_settings,
            settings::api::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Unifile");
}
