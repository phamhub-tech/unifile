use std::io::ErrorKind;

/// All errors that can occur during filesystem operations.
///
/// Variants are kept specific enough to let callers decide whether an error
/// is recoverable (e.g. skip a permission-denied entry and continue the walk)
/// or fatal (e.g. stop the scan entirely).
#[derive(Debug, thiserror::Error)]
pub enum FsError {
    /// The process does not have permission to access this path.
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    /// The path no longer exists on the filesystem.
    ///
    /// This can happen during a scan when a file is deleted between
    /// the directory listing and the `stat` call (TOCTOU race).
    #[error("Path not found: {path}")]
    NotFound { path: String },

    /// A general IO error occurred at the given path.
    ///
    /// This variant intentionally has **no** `#[from]` attribute.
    /// IO errors must always carry a path for context so that log messages
    /// are actionable. Use [`FsError::from_io`] at call sites instead of `?`.
    #[error("IO error at '{path}': {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// An ignore-pattern (glob) could not be compiled or applied.
    ///
    /// `#[from]` is safe here because `ignore::Error` has only one sensible
    /// mapping, and the context (which pattern failed) is already embedded
    /// in the `ignore::Error` message.
    #[error("Failed to build path overrides: {0}")]
    OverrideBuild(#[from] ignore::Error),

    /// The IPC channel to the frontend was closed while a scan was in progress.
    #[error("Scan interrupted: IPC channel closed")]
    ChannelClosed,

    /// The blocking scan task panicked on the thread pool.
    #[error("Scan task panicked")]
    TaskPanic,
}

impl FsError {
    /// Promotes a bare [`std::io::Error`] into an [`FsError`] by attaching
    /// the affected path as context.
    ///
    /// Use this whenever you have an IO error and a path but don't need to
    /// classify the error kind.
    pub fn io(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    /// Classifies an [`std::io::Error`] by its kind and converts it into the
    /// most specific [`FsError`] variant.
    ///
    /// This is the preferred way to turn an `io::Error` into an `FsError`
    /// because it maps `PermissionDenied` and `NotFound` to named variants,
    /// letting callers handle them with a simple `match` rather than checking
    /// `std::io::ErrorKind` everywhere.
    ///
    /// # Example
    ///
    /// ```rust
    /// let metadata = fs::metadata(&path)
    ///     .map_err(|e| FsError::from_io(path.display().to_string(), e))?;
    /// ```
    pub fn from_io(path: impl Into<String>, e: std::io::Error) -> Self {
        let path = path.into();
        match e.kind() {
            ErrorKind::PermissionDenied => Self::PermissionDenied { path },
            ErrorKind::NotFound => Self::NotFound { path },
            _ => Self::io(path, e),
        }
    }
}

/// type alias for Result<T, FsError>
pub type FsResult<T> = core::result::Result<T, FsError>;
