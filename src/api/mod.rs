use axum::Router;
use daoyi_cloud_common::app::AppState;
use daoyi_module_demo::demo;

pub fn create_router() -> Router<AppState> {
    Router::new().merge(demo::create_router())
}

// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> Response {
//     }
// }
