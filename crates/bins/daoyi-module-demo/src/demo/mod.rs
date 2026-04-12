use daoyi_cloud_common::auth::jwt::middleware::JwtAuthHandler;
use salvo::oapi::RouterExt;
use salvo::oapi::SecurityRequirement;
use salvo::prelude::*;

pub mod admin_api;

pub fn create_router() -> Router {
    let admin_router = admin_api::create_router();
    Router::new().push(
        Router::with_path("/admin-api")
            .hoop(JwtAuthHandler::new())
            .oapi_security(SecurityRequirement::new::<&str, [&str; 0], &str>(
                "bearer_auth",
                [],
            ))
            .oapi_security(SecurityRequirement::new::<&str, [&str; 0], &str>(
                "tenant_id",
                [],
            ))
            .push(admin_router),
    )
}
