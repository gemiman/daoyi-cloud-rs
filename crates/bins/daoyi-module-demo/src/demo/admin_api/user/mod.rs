use axum::{Router, debug_handler, routing};
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::extract::path::Path;
use daoyi_cloud_common::extract::valid::{ValidJson, ValidQuery};
use daoyi_cloud_common::pojo::pagination::PageResult;
use daoyi_cloud_common::response::CommonResult;
use daoyi_cloud_common::success;
use daoyi_entity_demo::demo::entity::sys_user;
use daoyi_entity_demo::demo::models::sys_user::{UserParams, UserQueryParams};
use daoyi_entity_demo::demo::service::sys_user_service;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(find_page))
        .route("/", routing::post(create))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::get(get_user_by_id))
        .route("/{id}", routing::delete(delete))
}

#[debug_handler]
async fn delete(Path(id): Path<String>) -> CommonResult<bool> {
    let result = sys_user_service::delete_user_by_id(id).await?;
    success!(result)
}

#[debug_handler]
async fn get_user_by_id(Path(id): Path<String>) -> CommonResult<Option<sys_user::Model>> {
    let user = sys_user_service::get_user_by_id(id).await?;
    success!(user)
}

#[debug_handler]
async fn update(
    Path(id): Path<String>,
    ValidJson(params): ValidJson<UserParams>,
) -> CommonResult<bool> {
    let result = sys_user_service::update_user_by_id(id, params).await?;
    success!(result)
}
#[debug_handler]
async fn create(ValidJson(params): ValidJson<UserParams>) -> CommonResult<sys_user::Model> {
    let user = sys_user_service::create_user(params).await?;
    success!(user)
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
