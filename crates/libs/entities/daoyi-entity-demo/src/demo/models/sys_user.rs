use crate::demo::entity::sys_user::ActiveModel;
use daoyi_cloud_common::constants::default_values::default_true;
use daoyi_cloud_common::extract::validations::validate_mobile_phone;
use daoyi_cloud_common::pojo::pagination::PageParam;
use sea_orm::DeriveIntoActiveModel;
use sea_orm::prelude::Date;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    pub keyword: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PageParam,
}

#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    #[validate(length(min = 1, max = 16, message = "用户姓名长度必须在 1 到 16 之间"))]
    pub name: String,
    pub gender: String,
    #[validate(length(min = 1, max = 16, message = "账号长度必须在 1 到 16 之间"))]
    pub account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度必须在 6 到 16 之间"))]
    pub password: String,
    #[validate(custom(function = "validate_mobile_phone"))]
    pub mobile_phone: String,
    pub birthday: Date,
    #[serde(default = "default_true")]
    pub enabled: bool,
}
