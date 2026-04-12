use crate::demo::entity::sys_user::ActiveModel;
use daoyi_cloud_common::constants::default_values::default_true;
use daoyi_cloud_common::constants::enumeration::Gender;
use daoyi_cloud_common::extract::validations::validate_mobile_phone;
use daoyi_cloud_common::pojo::pagination::PageParam;
use salvo::oapi::{ToParameters, ToSchema};
use sea_orm::DeriveIntoActiveModel;
use sea_orm::prelude::Date;
use serde::Deserialize;
use validator::Validate;

/// 用户查询参数
#[derive(Debug, Deserialize, Validate, ToParameters, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    /// 搜索关键词
    pub keyword: Option<String>,
    #[serde(flatten)]
    #[validate(nested)]
    pub pagination: PageParam,
}

/// 用户新增/编辑参数
#[derive(Debug, Deserialize, Validate, DeriveIntoActiveModel, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserParams {
    /// 用户姓名
    #[validate(length(min = 1, max = 16, message = "用户姓名长度必须在 1 到 16 之间"))]
    pub name: String,
    /// 性别
    pub gender: Gender,
    /// 登录账号
    #[validate(length(min = 1, max = 16, message = "账号长度必须在 1 到 16 之间"))]
    pub account: String,
    /// 登录密码
    #[validate(length(min = 6, max = 16, message = "密码长度必须在 6 到 16 之间"))]
    pub password: String,
    /// 手机号码
    #[validate(custom(function = "validate_mobile_phone"))]
    pub mobile_phone: String,
    /// 出生日期
    pub birthday: Date,
    /// 是否启用
    #[serde(default = "default_true")]
    pub enabled: bool,
}
