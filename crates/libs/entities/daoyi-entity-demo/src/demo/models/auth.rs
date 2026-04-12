use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginParams {
    #[validate(length(min = 1, max = 16, message = "账号长度必须在 1 到 16 之间"))]
    pub account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度必须在 6 到 16 之间"))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub access_token: String,
}
