use axum::Router;
use daoyi_cloud_common::app::AppState;
use daoyi_cloud_common::openapi::ApiDoc;
use daoyi_module_demo::demo;
use utoipa::OpenApi;

pub fn create_router() -> (Router<AppState>, utoipa::openapi::OpenApi) {
    let (demo_router, demo_api) = demo::create_router();
    let mut api = ApiDoc::openapi();
    api.merge(demo_api);
    (Router::new().merge(demo_router), api)
}

// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> Response {
//     }
// }
