use crate::utils::serde_utils::deserialize_number;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE_NO: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 10;

pub const PAGE_SIZE_NONE: u64 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    #[validate(range(min = 1, message = "页码最小值为 1"))]
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    #[validate(custom(function = "validate_page_size"))]
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
}
fn validate_page_size(page_size: u64) -> Result<(), validator::ValidationError> {
    match page_size {
        s if s < 1 => {
            let mut err = validator::ValidationError::new("page_size_range");
            err.message = Some("每页条数最小值为 1".into());
            Err(err)
        }
        s if s > 200 => {
            let mut err = validator::ValidationError::new("page_size_range");
            err.message = Some("每页条数最大值为 200".into());
            Err(err)
        }
        _ => Ok(()),
    }
}

fn default_page_no() -> u64 {
    DEFAULT_PAGE_NO
}

fn default_page_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    pub page_no: u64,
    pub page_size: u64,
    pub total: u64,
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
