use serde::Serialize;

/// The single wire type returned by every Tauri command.
///
/// On success, `error` is `false` and `data` holds the result.
/// On failure, `error` is `true`, `message` describes the problem, and
/// `data` is `None` (commands that can fail use `ApiResponse<Option<T>>`).
///
/// The frontend always checks `error` before reading `data`.
#[derive(Clone, Serialize)]
pub struct ApiResponse<T: Serialize> {
    /// `true` when the command failed.
    error: bool,
    /// Description of the outcome.
    message: String,
    /// The response payload. `None` when `error` is `true`.
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    /// Creates a successful response carrying `data`.
    pub fn ok(data: T) -> Self {
        Self {
            error: false,
            message: "Success".to_string(),
            data,
        }
    }
}

impl<T: Serialize> ApiResponse<Option<T>> {
    /// Creates an error response with the given message.
    ///
    /// `data` is always `None`, so the frontend receives `null` for that field.
    pub fn err(message: impl Into<String>) -> Self {
        Self {
            error: true,
            message: message.into(),
            data: None,
        }
    }
}

/// Converts any `Result<T, E>` into an `ApiResponse<Option<T>>`.
///
/// This is the bridge between controller/model code (which returns `Result`)
/// and command functions (which return `ApiResponse`).
/// 
/// Commands that can fail call `.into()` at the boundary instead of manually
/// constructing an `ApiResponse`:
///
/// ```rust
/// #[tauri::command]
/// pub fn get_entries(path: String) -> ApiResponse<Option<Vec<FSEntry>>> {
///     entries_inner(&path).into()
/// }
/// ```
impl<T, E> From<Result<T, E>> for ApiResponse<Option<T>>
where
    T: Serialize,
    E: std::fmt::Display,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(data) => Self::ok(Some(data)),
            Err(e) => Self::err(e.to_string()),
        }
    }
}
