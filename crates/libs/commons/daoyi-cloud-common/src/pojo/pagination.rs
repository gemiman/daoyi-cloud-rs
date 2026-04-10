use crate::utils::serde_utils::deserialize_number;
use serde::{Deserialize, Serialize};

const DEFAULT_PAGE_NO: u64 = 1;
const DEFAULT_PAGE_SIZE: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: u64,
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
