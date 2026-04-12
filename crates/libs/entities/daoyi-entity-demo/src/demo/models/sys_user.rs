use crate::demo::entity::sys_user::ActiveModel;
use daoyi_cloud_common::constants::default_values::{
    default_page_no, default_page_size, default_true,
};
use daoyi_cloud_common::constants::enumeration::Gender;
use daoyi_cloud_common::extract::validations::{validate_mobile_phone, validate_page_size};
use daoyi_cloud_common::pojo::pagination::PageParam;
use daoyi_cloud_common::utils::serde_utils::deserialize_number;
use salvo::oapi::{ToParameters, ToSchema};
use sea_orm::DeriveIntoActiveModel;
use sea_orm::prelude::Date;
use serde::Deserialize;
use validator::Validate;

/// 用户查询参数
#[derive(Debug, Deserialize, Validate, ToParameters, ToSchema)]
#[serde(rename_all = "camelCase")]
#[salvo(parameters(default_parameter_in = Query))]
pub struct UserQueryParams {
    /// 搜索关键词
    pub keyword: Option<String>,
    /// 页码
    #[validate(range(min = 1, message = "页码最小值为 1"))]
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    /// 每页条数
    #[validate(custom(function = "validate_page_size"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}

impl UserQueryParams {
    pub fn pagination(&self) -> PageParam {
        PageParam::new(self.page_no, self.page_size)
    }
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
