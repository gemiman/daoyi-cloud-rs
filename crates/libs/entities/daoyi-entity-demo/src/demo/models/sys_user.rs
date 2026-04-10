use daoyi_cloud_common::pojo::pagination::PageParam;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryParams {
    pub keyword: Option<String>,
    #[serde(flatten)]
    pub pagination: PageParam,
}
