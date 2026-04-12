use axum::Router;
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::auth;
use utoipa::openapi::OpenApi;

pub mod admin_api;

pub fn create_router() -> (Router<AppState>, OpenApi) {
    let (admin_router, admin_api) = admin_api::create_router();
    let router = Router::new()
        .nest("/admin-api", admin_router)
        .route_layer(auth::jwt::middleware::get_auth_layer());
    (router, admin_api)
}
