use axum::{Router, debug_handler, routing};
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::extract::valid::ValidQuery;
use daoyi_cloud_common::pojo::pagination::PageResult;
use daoyi_cloud_common::response::CommonResult;
use daoyi_cloud_common::success;
use daoyi_entity_demo::demo::entity::sys_user;
use daoyi_entity_demo::demo::models::sys_user::UserQueryParams;
use daoyi_entity_demo::demo::service::sys_user_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(find_page))
}

#[debug_handler]
async fn find_page(
    ValidQuery(params): ValidQuery<UserQueryParams>,
) -> CommonResult<PageResult<sys_user::Model>> {
    let users = sys_user_service::query_page(params).await?;
    success!(users)
}

#[debug_handler]
async fn query_users() -> CommonResult<Vec<sys_user::Model>> {
    let users = sys_user_service::query_users().await?;
    success!(users)
}
