pub mod latency;

use crate::conf::ServerConfig;
use salvo::cors::{Any, Cors};
use salvo::oapi::OpenApi;
use salvo::oapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme};
use salvo::prelude::*;
use salvo::trailing_slash::{TrailingSlash, TrailingSlashAction};
use salvo_oapi::scalar::Scalar;
use salvo_oapi::swagger_ui::SwaggerUi;
use std::time::Duration;

pub struct AppServer {
    config: &'static ServerConfig,
}

impl AppServer {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, router: Router) -> anyhow::Result<()> {
        let port = self.config.port();

        // 创建 OpenAPI 文档
        let doc = OpenApi::new("DaoYi Cloud API", "0.9.0")
            .add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer).bearer_format("JWT")),
            )
            .add_security_scheme(
                "tenant_id",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                    "tenant-id",
                    "租户ID",
                ))),
            )
            .merge_router(&router);

        let router = router
            .push(doc.into_router("/api-docs/openapi.json"))
            .push(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json")
                    .into_router("/swagger-ui"),
            )
            .push(Scalar::new("/api-docs/openapi.json").into_router("/scalar"))
            .push(Router::new().get(index))
            .hoop(TrailingSlash::new(TrailingSlashAction::Remove));

        // CORS 必须加到 Service 级别
        let cors = Cors::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(43200))
            .into_handler();

        let service = Service::new(router).hoop(cors);

        let listener = TcpListener::new(("0.0.0.0", port)).bind().await;
        tracing::info!("listening on http://0.0.0.0:{}", port);
        tracing::info!("Swagger UI: http://localhost:{}/swagger-ui", port);
        tracing::info!("Scalar: http://localhost:{}/scalar", port);

        Server::new(listener).serve(service).await;
        Ok(())
    }
}

#[handler]
async fn index(res: &mut Response) {
    crate::success!(res, "Hello DaoYi Cloud Rust!");
}
