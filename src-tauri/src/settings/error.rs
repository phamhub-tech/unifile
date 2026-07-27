/// All errors that can occur while loading, saving, or watching app settings.
#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    /// The settings [`Mutex`] was poisoned because a thread panicked while
    /// holding it.
    ///
    /// [`Mutex`]: std::sync::Mutex
    #[error("Settings lock is poisoned")]
    LockPoisoned,

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

    /// The file-system watcher could not be initialised or encountered a
    /// runtime error.
    ///
    /// `#[from]` lets `?` convert `notify::Error` automatically.
    #[error("File watcher error: {0}")]
    Watcher(#[from] notify::Error),
}
