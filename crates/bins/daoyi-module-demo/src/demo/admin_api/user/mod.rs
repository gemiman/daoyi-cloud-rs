use axum::{Router, debug_handler, routing};
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::extract::path::Path;
use daoyi_cloud_common::extract::valid::{ValidJson, ValidQuery};
use daoyi_cloud_common::pojo::pagination::PageResult;
use daoyi_cloud_common::response::{ApiResponse, CommonResult};
use daoyi_cloud_common::success;
use daoyi_entity_demo::demo::entity::sys_user;
use daoyi_entity_demo::demo::models::sys_user::{UserParams, UserQueryParams};
use daoyi_entity_demo::demo::service::sys_user_service;
use utoipa::OpenApi as OpenApiDerive;
use utoipa::openapi::OpenApi;

#[derive(OpenApiDerive)]
#[openapi(
    paths(query_users, find_page, create, update, get_user_by_id, delete),
    components(schemas(
        UserParams, UserQueryParams, sys_user::Model, PageResult<sys_user::Model>,
        ApiResponse<sys_user::Model>, ApiResponse<Vec<sys_user::Model>>,
        ApiResponse<PageResult<sys_user::Model>>, ApiResponse<bool>
    ))
)]
struct ApiDoc;

pub fn create_router() -> (Router<AppState>, OpenApi) {
    let router = Router::new()
        .route("/", routing::get(query_users))
        .route("/page", routing::get(find_page))
        .route("/", routing::post(create))
        .route("/{id}", routing::put(update))
        .route("/{id}", routing::get(get_user_by_id))
        .route("/{id}", routing::delete(delete));
    (router, ApiDoc::openapi())
}

#[utoipa::path(
    delete,
    path = "/admin-api/demo/users/{id}",
    params(("id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "Delete user successfully", body = ApiResponse<bool>),
    ),
)]
#[debug_handler]
async fn delete(Path(id): Path<String>) -> CommonResult<bool> {
    let result = sys_user_service::delete_user_by_id(id).await?;
    success!(result)
}

#[utoipa::path(
    get,
    path = "/admin-api/demo/users/{id}",
    params(("id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "Get user by ID", body = ApiResponse<sys_user::Model>),
    ),
)]
#[debug_handler]
async fn get_user_by_id(Path(id): Path<String>) -> CommonResult<Option<sys_user::Model>> {
    let user = sys_user_service::get_user_by_id(id).await?;
    success!(user)
}

#[utoipa::path(
    put,
    path = "/admin-api/demo/users/{id}",
    params(("id" = String, Path, description = "User ID")),
    request_body = UserParams,
    responses(
        (status = 200, description = "Update user successfully", body = ApiResponse<bool>),
    ),
)]
#[debug_handler]
async fn update(
    Path(id): Path<String>,
    ValidJson(params): ValidJson<UserParams>,
) -> CommonResult<bool> {
    let result = sys_user_service::update_user_by_id(id, params).await?;
    success!(result)
}

#[utoipa::path(
    post,
    path = "/admin-api/demo/users",
    request_body = UserParams,
    responses(
        (status = 200, description = "Create user successfully", body = ApiResponse<sys_user::Model>),
    ),
)]
#[debug_handler]
async fn create(ValidJson(params): ValidJson<UserParams>) -> CommonResult<sys_user::Model> {
    let user = sys_user_service::create_user(params).await?;
    success!(user)
}

#[utoipa::path(
    get,
    path = "/admin-api/demo/users/page",
    params(UserQueryParams),
    responses(
        (status = 200, description = "Query users by page", body = ApiResponse<PageResult<sys_user::Model>>),
    ),
)]
#[debug_handler]
async fn find_page(
    ValidQuery(params): ValidQuery<UserQueryParams>,
) -> CommonResult<PageResult<sys_user::Model>> {
    let users = sys_user_service::query_page(params).await?;
    success!(users)
}

#[utoipa::path(
    get,
    path = "/admin-api/demo/users",
    responses(
        (status = 200, description = "Query all users", body = ApiResponse<Vec<sys_user::Model>>),
    ),
)]
#[debug_handler]
async fn query_users() -> CommonResult<Vec<sys_user::Model>> {
    let users = sys_user_service::query_users().await?;
    success!(users)
}
