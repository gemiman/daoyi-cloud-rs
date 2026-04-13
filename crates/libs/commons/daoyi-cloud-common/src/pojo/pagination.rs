use crate::constants::default_values::default_page_no;
use crate::constants::default_values::default_page_size;
use crate::extract::validations::validate_page_size;
use crate::utils::serde_utils::deserialize_number;
use salvo::oapi::{ToParameters, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 分页查询参数
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate, ToSchema, ToParameters)]
#[serde(rename_all = "camelCase")]
#[salvo(parameters(default_parameter_in = Query))]
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

/// 定义一个分页查询参数结构体，自动包含 `page_no` 和 `page_size` 字段及 `pagination()` 方法。
///
/// # 用法
/// ```ignore
/// page_query_params! {
///     /// 用户查询参数
///     pub struct UserQueryParams {
///         /// 搜索关键词
///         pub keyword: Option<String>,
///     }
/// }
/// ```
/// 展开后等价于：
/// ```ignore
/// #[derive(Debug, Deserialize, Validate, ToParameters, ToSchema)]
/// #[serde(rename_all = "camelCase")]
/// #[salvo(parameters(default_parameter_in = Query))]
/// /// 用户查询参数
/// pub struct UserQueryParams {
///     /// 搜索关键词
///     pub keyword: Option<String>,
///     /// 页码
///     #[validate(range(min = 1, message = "页码最小值为 1"))]
///     #[serde(default = "...", deserialize_with = "...")]
///     pub page_no: u64,
///     /// 每页条数
///     #[validate(custom(function = "..."))]
///     #[serde(default = "...", deserialize_with = "...")]
///     pub page_size: u64,
/// }
/// impl UserQueryParams {
///     pub fn pagination(&self) -> PageParam { ... }
/// }
/// ```
#[macro_export]
macro_rules! page_query_params {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($(#[$field_meta:meta])* $field_vis:vis $field_name:ident : $field_ty:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, ::serde::Deserialize, ::validator::Validate, ::salvo::oapi::ToParameters, ::salvo::oapi::ToSchema)]
        #[serde(rename_all = "camelCase")]
        #[salvo(parameters(default_parameter_in = Query))]
        $vis struct $name {
            $($(#[$field_meta])* $field_vis $field_name: $field_ty,)*
            /// 页码
            #[validate(range(min = 1, message = "页码最小值为 1"))]
            #[serde(default = "daoyi_cloud_common::constants::default_values::default_page_no", deserialize_with = "daoyi_cloud_common::utils::serde_utils::deserialize_number")]
            pub page_no: u64,
            /// 每页条数
            #[validate(custom(function = "daoyi_cloud_common::extract::validations::validate_page_size"))]
            #[serde(default = "daoyi_cloud_common::constants::default_values::default_page_size", deserialize_with = "daoyi_cloud_common::utils::serde_utils::deserialize_number")]
            pub page_size: u64,
        }

        impl $name {
            pub fn pagination(&self) -> daoyi_cloud_common::pojo::pagination::PageParam {
                daoyi_cloud_common::pojo::pagination::PageParam::new(self.page_no, self.page_size)
            }
        }
    };
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
