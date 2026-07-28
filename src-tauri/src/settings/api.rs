use tauri::State;

use unifile_core::settings::{AppSettings, SettingsError};

use crate::api::ApiResponse;

use super::models::AppSettingsManager;

/// Returns the current app settings.
///
/// Acquires the settings mutex and clones the value. Returns an error
/// response if the mutex is poisoned (another thread panicked while holding it).
#[tauri::command]
pub fn get_settings(manager: State<AppSettingsManager>) -> ApiResponse<Option<AppSettings>> {
    manager
        .settings
        .lock()
        .map(|guard| guard.clone())
        .map_err(|_| SettingsError::LockPoisoned)
        .into()
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
    manager.save(&new_settings).into()
}
