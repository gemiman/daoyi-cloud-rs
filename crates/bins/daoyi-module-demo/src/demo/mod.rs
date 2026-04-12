use daoyi_cloud_common::auth::jwt::middleware::JwtAuthHandler;
use salvo::prelude::*;

pub mod admin_api;

pub fn create_router() -> Router {
    let admin_router = admin_api::create_router();
    Router::new().push(
        Router::with_path("/admin-api")
            .hoop(JwtAuthHandler::new())
            .push(admin_router),
    )
}
