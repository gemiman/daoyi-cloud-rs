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
    tag = "用户管理",
    operation_id = "deleteUser",
    summary = "删除用户",
    description = "根据用户ID删除指定用户",
    params(("id" = String, Path, description = "用户ID")),
    responses(
        (status = 200, description = "删除成功", body = ApiResponse<bool>),
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
    tag = "用户管理",
    operation_id = "getUserById",
    summary = "根据ID查询用户",
    description = "根据用户ID获取用户详细信息",
    params(("id" = String, Path, description = "用户ID")),
    responses(
        (status = 200, description = "查询成功，返回用户信息", body = ApiResponse<sys_user::Model>),
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
    tag = "用户管理",
    operation_id = "updateUser",
    summary = "更新用户",
    description = "根据用户ID更新用户信息",
    params(("id" = String, Path, description = "用户ID")),
    request_body = UserParams,
    responses(
        (status = 200, description = "更新成功", body = ApiResponse<bool>),
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
    tag = "用户管理",
    operation_id = "createUser",
    summary = "创建用户",
    description = "创建新用户",
    request_body = UserParams,
    responses(
        (status = 200, description = "创建成功，返回用户信息", body = ApiResponse<sys_user::Model>),
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
    tag = "用户管理",
    operation_id = "findUserPage",
    summary = "分页查询用户",
    description = "根据关键词和分页参数分页查询用户列表",
    params(UserQueryParams),
    responses(
        (status = 200, description = "查询成功，返回分页结果", body = ApiResponse<PageResult<sys_user::Model>>),
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
    tag = "用户管理",
    operation_id = "queryUsers",
    summary = "查询所有用户",
    description = "查询系统中所有用户列表",
    responses(
        (status = 200, description = "查询成功，返回用户列表", body = ApiResponse<Vec<sys_user::Model>>),
    ),
)]
#[debug_handler]
async fn query_users() -> CommonResult<Vec<sys_user::Model>> {
    let users = sys_user_service::query_users().await?;
    success!(users)
}
