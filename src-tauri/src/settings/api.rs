use tauri::State;

use crate::api::ApiResponse;

use super::controller;
use super::models::{AppSettings, AppSettingsManager};

/// Returns the current app settings.
///
/// Acquires the settings mutex and clones the value. Returns an error
/// response if the mutex is poisoned (another thread panicked while holding it).
#[tauri::command]
pub fn get_settings(manager: State<AppSettingsManager>) -> ApiResponse<Option<AppSettings>> {
    controller::get_settings(&manager).into()
}

/// Writes `new_settings` to disk.
///
/// The in-memory settings are updated asynchronously by the file-watcher
/// thread, not by this command directly.
#[tauri::command]
pub fn save_settings(
    manager: State<AppSettingsManager>,
    new_settings: AppSettings,
) -> ApiResponse<Option<()>> {
    controller::save_settings(&manager, new_settings).into()
}
