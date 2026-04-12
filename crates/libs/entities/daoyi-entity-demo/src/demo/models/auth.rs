use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 登录请求参数
#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginParams {
    /// 登录账号
    #[validate(length(min = 1, max = 16, message = "账号长度必须在 1 到 16 之间"))]
    pub account: String,
    /// 登录密码
    #[validate(length(min = 6, max = 16, message = "密码长度必须在 6 到 16 之间"))]
    pub password: String,
}

/// 登录结果
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    /// JWT 访问令牌
    pub access_token: String,
}
