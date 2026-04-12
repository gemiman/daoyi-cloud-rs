use axum::Router;
use daoyi_cloud_common::app::AppState;
use utoipa::openapi::OpenApi;

pub mod auth;
pub mod user;

pub fn create_router() -> (Router<AppState>, OpenApi) {
    let (auth_router, auth_api) = auth::create_router();
    let (user_router, user_api) = user::create_router();

    let router = Router::new().nest(
        "/demo",
        Router::new()
            .nest("/users", user_router)
            .nest("/auth", auth_router),
    );

    let mut api = auth_api;
    api.merge(user_api);

    (router, api)
}
