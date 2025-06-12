use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ApiResponse<T> {
    error: bool,
    message: String,
    pub data: T,
}
impl<T> ApiResponse<T> {
    pub fn new(data: T, message: String, error: bool) -> Self {
        Self {
            error,
            message,
            data,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct ApiError {
    error: bool,
    message: String,
}
impl ApiError {
    pub fn new(message: String) -> Self {
        Self {
            error: true,
            message,
        }
    }
}

#[macro_export]
macro_rules! api_response {
    ($data: expr) => {
        ApiResponse::new($data, "Success".to_string(), false)
    };
    ($data: expr, $msg: expr) => {
        ApiResponse::new($data, $msg, false)
    };
    ($data: expr, $msg: expr, $error: ident) => {
        ApiResponse::new($data, $msg, $error)
    };
}

#[macro_export]
macro_rules! api_error {
    ($msg: expr) => {
        ApiError::new($msg)
    };
}
