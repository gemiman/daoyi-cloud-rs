use crate::app::AppState;
use axum::Router;

pub mod user;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/api", Router::new().nest("/users", user::create_router()))
}

// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> Response {
//     }
// }
