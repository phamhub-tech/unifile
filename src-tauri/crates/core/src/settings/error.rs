/// All errors that can occur while loading, saving, or watching app settings.
#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    /// A filesystem IO error occurred while reading or writing the settings file.
    ///
    /// `#[from]` lets `?` convert `std::io::Error` automatically in functions
    /// that return `Result<_, SettingsError>`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// The settings JSON could not be parsed into [`AppSettings`].
    ///
    /// `#[from]` lets `?` convert `serde_json::Error` automatically.
    ///
    /// [`AppSettings`]: crate::settings::models::AppSettings
    #[error("Failed to parse settings: {0}")]
    Deserialization(#[from] serde_json::Error),
}
