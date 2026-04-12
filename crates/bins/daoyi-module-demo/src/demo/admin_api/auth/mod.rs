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
    request_body = LoginParams,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<LoginResult>),
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
    responses(
        (status = 200, description = "Get current user info", body = ApiResponse<Principal>),
    ),
)]
#[debug_handler]
async fn get_user_info(Extension(principal): Extension<Principal>) -> CommonResult<Principal> {
    success!(principal)
}
