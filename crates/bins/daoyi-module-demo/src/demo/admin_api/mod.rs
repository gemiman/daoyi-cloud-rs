use axum::Router;
use daoyi_cloud_common::app::AppState;

pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/demo", Router::new().nest("/users", user::create_router()))
}
