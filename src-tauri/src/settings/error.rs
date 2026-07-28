use unifile_core::settings::SettingsError;

/// Errors that can occur in the Tauri settings command layer.
///
/// This type wraps [`SettingsError`] for domain-level failures and adds
/// [`ManagerError::LockPoisoned`] for the concurrency concern that belongs
/// here — the `AppSettingsManager` owns the `Mutex`, so only this layer
/// can ever observe a poisoned lock.
#[derive(Debug, thiserror::Error)]
pub enum ManagerError {
    /// The settings [`Mutex`] was poisoned because a thread panicked while
    /// holding it.
    ///
    /// [`Mutex`]: std::sync::Mutex
    #[error("Settings lock is poisoned")]
    LockPoisoned,

    /// A domain-level settings error (IO, parse failure, watcher error).
    ///
    /// `#[from]` lets `?` convert [`SettingsError`] automatically in functions
    /// that return `Result<_, ManagerError>`.
    #[error(transparent)]
    Settings(#[from] SettingsError),
}
