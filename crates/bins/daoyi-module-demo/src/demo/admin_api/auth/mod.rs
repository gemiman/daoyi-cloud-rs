use axum::extract::ConnectInfo;
use axum::{Extension, Router, debug_handler, routing};
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::auth::jwt::Principal;
use daoyi_cloud_common::extract::valid::ValidJson;
use daoyi_cloud_common::response::{ApiResponse, CommonResult};
use daoyi_cloud_common::success;
use daoyi_entity_demo::demo::models::auth::{LoginParams, LoginResult};
use daoyi_entity_demo::demo::service::auth_service;
use std::net::SocketAddr;
use utoipa::OpenApi as OpenApiDerive;
use utoipa::openapi::OpenApi;

#[derive(OpenApiDerive)]
#[openapi(
    paths(login, get_user_info),
    components(schemas(LoginParams, LoginResult, Principal, ApiResponse<LoginResult>, ApiResponse<Principal>
    ))
)]
struct ApiDoc;

pub fn create_router() -> (Router<AppState>, OpenApi) {
    let router = Router::new()
        .route("/login", routing::post(login))
        .route("/user-info", routing::get(get_user_info));
    (router, ApiDoc::openapi())
}

#[utoipa::path(
    post,
    path = "/admin-api/demo/auth/login",
    tag = "认证管理",
    operation_id = "login",
    summary = "用户登录",
    description = "通过账号密码登录系统，返回 JWT 访问令牌",
    request_body = LoginParams,
    responses(
        (status = 200, description = "登录成功，返回访问令牌", body = ApiResponse<LoginResult>),
    ),
    security(())
)]
#[debug_handler]
#[tracing::instrument(
    name = "login",
    skip_all,
    fields(account = %params.account, ip = %_addr.ip())
)]
async fn login(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> CommonResult<LoginResult> {
    tracing::info!("login: {:?}", params);
    let result = auth_service::login(params).await?;
    tracing::info!("login result: {:?}", result);
    success!(result)
}

#[utoipa::path(
    get,
    path = "/admin-api/demo/auth/user-info",
    tag = "认证管理",
    operation_id = "getUserInfo",
    summary = "获取当前用户信息",
    description = "根据 JWT 令牌获取当前登录用户的基本信息",
    responses(
        (status = 200, description = "获取成功，返回用户信息", body = ApiResponse<Principal>),
    ),
)]
#[debug_handler]
async fn get_user_info(Extension(principal): Extension<Principal>) -> CommonResult<Principal> {
    success!(principal)
}
