use axum::Router;
use daoyi_cloud_common::app::AppState;

pub mod admin_api;
pub fn create_router() -> Router<AppState> {
    Router::new().nest("/admin-api", admin_api::create_router())
}
