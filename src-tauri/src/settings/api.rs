use tauri::State;

use crate::{
    api::{ApiError, ApiResponse},
    api_error, api_response,
};

use super::models::{AppSettings, AppSettingsManager};

#[tauri::command]
pub fn get_settings(
    manager: State<AppSettingsManager>,
) -> Result<ApiResponse<AppSettings>, ApiError> {
    let guard = manager
        .settings
        .lock()
        .map_err(|e| api_error!(e.to_string()))?;
    let settings = guard.clone();

    Ok(api_response!(settings))
}

#[tauri::command]
pub fn save_settings(
    manager: State<AppSettingsManager>,
    new_settings: AppSettings,
) -> Result<ApiResponse<String>, ApiError> {
    manager
        .save(&new_settings)
        .map_err(|e| api_error!(e.to_string()))?;

    Ok(api_response!("Settings saved".to_string()))
}
