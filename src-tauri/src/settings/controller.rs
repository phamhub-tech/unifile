use super::error::SettingsError;
use super::models::{AppSettings, AppSettingsManager};

/// Inner logic for [`get_settings`], returning a typed `Result`.
pub fn get_settings(manager: &AppSettingsManager) -> Result<AppSettings, SettingsError> {
    let guard = manager
        .settings
        .lock()
        .map_err(|_| SettingsError::LockPoisoned)?;
    Ok(guard.clone())
}

/// Writes `new_settings` to disk.
pub fn save_settings(
    manager: &AppSettingsManager,
    new_settings: AppSettings,
) -> Result<(), SettingsError> {
    manager.save(&new_settings)
}
