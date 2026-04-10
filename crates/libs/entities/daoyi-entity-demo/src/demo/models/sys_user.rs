use daoyi_cloud_common::pojo::pagination::PageParam;
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
