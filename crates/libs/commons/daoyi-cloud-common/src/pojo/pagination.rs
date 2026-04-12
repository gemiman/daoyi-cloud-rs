use crate::constants::default_values::default_page_no;
use crate::constants::default_values::default_page_size;
use crate::extract::validations::validate_page_size;
use crate::utils::serde_utils::deserialize_number;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    /// 页码
    #[validate(range(min = 1, message = "页码最小值为 1"))]
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    /// 每页条数
    #[validate(custom(function = "validate_page_size"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}

impl PageParam {
    pub fn new(page_no: u64, page_size: u64) -> Self {
        Self { page_no, page_size }
    }
}

/// 分页查询结果
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    /// 页码
    pub page_no: u64,
    /// 每页条数
    pub page_size: u64,
    /// 总记录数
    pub total: u64,
    /// 数据列表
    pub list: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn new(page_no: u64, page_size: u64, total: u64, list: Vec<T>) -> Self {
        Self {
            page_no,
            page_size,
            total,
            list,
        }
    }

    pub fn from_pagination(pagination: PageParam, total: u64, list: Vec<T>) -> Self {
        Self::new(pagination.page_no, pagination.page_size, total, list)
    }
}
