use axum::Router;
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::auth;

pub mod admin_api;
pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin-api", admin_api::create_router())
        .route_layer(auth::jwt::middleware::get_auth_layer())
}
