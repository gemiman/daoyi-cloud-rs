use salvo::oapi::ToSchema;
use salvo::prelude::*;
use salvo::writing::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

pub type CommonResult<T> = Result<ApiResponse<T>, ApiError>;

/// 统一 API 响应结构
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T: Serialize + ToSchema + Send> {
    /// 状态码，0 表示成功
    pub code: i32,
    /// 提示信息
    pub msg: String,
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize + ToSchema + Send> ApiResponse<T> {
    pub fn new<M: AsRef<str>>(code: i32, msg: M, data: Option<T>) -> Self {
        Self {
            code,
            msg: String::from(msg.as_ref()),
            data,
        }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(0, "", data)
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

/// 将 ApiResponse 写入 salvo Response
pub fn write_json_response<T: Serialize + ToSchema + Send>(
    res: &mut Response,
    data: ApiResponse<T>,
) {
    res.status_code(StatusCode::OK);
    res.render(Json(data));
}

/// 将 ApiError 写入 salvo Response
pub fn write_error_response(res: &mut Response, error: ApiError) {
    let status = error.status_code();
    let body = ApiResponse::<()>::err_msg(error.to_string());
    res.status_code(status);
    res.render(Json(body));
}

#[macro_export]
macro_rules! success {
    ($res:expr, $data:expr) => {
        $crate::response::write_json_response($res, $crate::response::ApiResponse::ok(Some($data)))
    };
    ($res:expr) => {
        $crate::response::write_json_response($res, $crate::response::ApiResponse::<()>::ok(None))
    };
}

pub fn _inner_success<T: Serialize + ToSchema + Send>(data: Option<T>) -> CommonResult<T> {
    Ok(ApiResponse::ok(data))
}
