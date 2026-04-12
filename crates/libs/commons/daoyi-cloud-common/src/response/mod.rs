use crate::error::ApiResult;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type CommonResult<T> = ApiResult<ApiResponse<T>>;

/// 统一 API 响应结构
#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[schema(title = "ApiResponse", description = "统一 API 响应结构")]
pub struct ApiResponse<T: Serialize + utoipa::ToSchema> {
    /// 状态码，0 表示成功
    pub code: i32,
    /// 提示信息
    pub msg: String,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize + utoipa::ToSchema> ApiResponse<T> {
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

impl<T: Serialize + utoipa::ToSchema> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

#[macro_export]
macro_rules! success {
    ($data:expr) => {
        $crate::response::_inner_success(Some($data))
    };
    () => {
        $crate::response::_inner_success(None::<()>)
    };
}

pub fn _inner_success<T: Serialize + utoipa::ToSchema>(data: Option<T>) -> CommonResult<T> {
    Ok(ApiResponse::ok(data))
}
