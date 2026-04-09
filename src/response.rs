use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new<M: AsRef<str>>(code: i32, msg: M, data: Option<T>) -> Self {
        Self {
            code,
            msg: String::from(msg.as_ref()),
            data,
        }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(0, "".to_string(), data)
    }

    pub fn err<M: AsRef<str>>(code: i32, msg: M) -> Self {
        Self::new(code, msg, None)
    }

    pub fn err_msg<M: AsRef<str>>(msg: M) -> Self {
        Self::err(1, msg)
    }

    pub fn ok_empty() -> Self {
        Self::new(0, "", None)
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}
