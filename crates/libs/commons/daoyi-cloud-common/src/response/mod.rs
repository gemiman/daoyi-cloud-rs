use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

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
    /// 请求追踪 ID（用于日志关联）
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub request_id: String,
}

impl<T: Serialize + ToSchema + Send> ApiResponse<T> {
    pub fn new<M: AsRef<str>>(code: i32, msg: M, data: Option<T>) -> Self {
        Self {
            code,
            msg: String::from(msg.as_ref()),
            data,
            request_id: String::new(),
        }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(0, "操作成功", data)
    }

    pub fn ok_with_request_id(data: Option<T>, request_id: String) -> Self {
        Self {
            code: 0,
            msg: String::from("操作成功"),
            data,
            request_id,
        }
    }

    pub fn err<M: AsRef<str>>(code: i32, msg: M) -> Self {
        Self::new(code, msg, None)
    }

    pub fn err_msg<M: AsRef<str>>(msg: M) -> Self {
        Self::err(1, msg)
    }

    pub fn ok_empty() -> Self {
        Self::new(0, "操作成功", None)
    }

    /// 设置请求追踪 ID
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = request_id;
        self
    }
}

/// 将成功响应写入 Salvo Response（等价于旧的 success! 宏）
#[macro_export]
macro_rules! json_ok {
    ($res:expr, $data:expr) => {
        $res.render(salvo::writing::Json($crate::response::ApiResponse::ok(
            Some($data),
        )))
    };
    ($res:expr) => {
        $res.render(salvo::writing::Json(
            $crate::response::ApiResponse::<()>::ok(None),
        ))
    };
}
