pub mod headers;
pub mod latency;
pub mod metrics;
pub mod ratelimit;

use crate::conf::ServerConfig;
use crate::conf::server::CorsConfig;
use crate::context::{AppContext, InjectContext};
use crate::db;
use crate::server::headers::SecurityHeadersMiddleware;
use crate::server::metrics::MetricsMiddleware;
use crate::server::ratelimit::RateLimitMiddleware;
use salvo::cors::{AllowOrigin, Cors, CorsHandler};
use salvo::http::Method;
use salvo::http::header::HeaderValue;
use salvo::oapi::OpenApi;
use salvo::oapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme};
use salvo::prelude::*;
use salvo::trailing_slash::{TrailingSlash, TrailingSlashAction};
use salvo_oapi::scalar::Scalar;
use salvo_oapi::swagger_ui::SwaggerUi;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: u64 = 30;
const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

pub struct AppServer {
    config: &'static ServerConfig,
    ctx: Arc<AppContext>,
    shutdown_flag: Arc<AtomicBool>,
}

impl Clone for AppServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config,
            ctx: self.ctx.clone(),
            shutdown_flag: self.shutdown_flag.clone(),
        }
    }
}

impl AppServer {
    pub fn new(config: &'static ServerConfig, ctx: Arc<AppContext>) -> Self {
        Self {
            config,
            ctx,
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn trigger_shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);
        tracing::info!("Shutdown signal received, initiating graceful shutdown...");
    }

    fn build_cors(cors_cfg: &CorsConfig) -> CorsHandler {
        let mut cors = Cors::new();
        if cors_cfg.allowed_origins.is_empty() {
            cors = cors.allow_origin(AllowOrigin::any());
        } else {
            let origins: Vec<HeaderValue> = cors_cfg
                .allowed_origins
                .iter()
                .filter_map(|o| o.parse::<HeaderValue>().ok())
                .collect();
            cors = cors.allow_origin(AllowOrigin::list(origins));
        }
        if cors_cfg.allowed_methods.is_empty() {
            cors = cors.allow_methods(salvo::cors::Any);
        } else {
            let methods: Vec<Method> = cors_cfg
                .allowed_methods
                .iter()
                .filter_map(|m| m.parse::<Method>().ok())
                .collect();
            cors = cors.allow_methods(methods);
        }
        if cors_cfg.allowed_headers.is_empty() {
            cors = cors.allow_headers(salvo::cors::Any);
        } else {
            let headers: Vec<salvo::http::HeaderName> = cors_cfg
                .allowed_headers
                .iter()
                .filter_map(|h| h.parse::<salvo::http::HeaderName>().ok())
                .collect();
            cors = cors.allow_headers(headers);
        }
        cors = cors
            .allow_credentials(cors_cfg.allow_credentials)
            .max_age(Duration::from_secs(cors_cfg.max_age_secs));
        cors.into_handler()
    }

    pub async fn start(&self, router: Router) -> anyhow::Result<()> {
        let port = self.config.port();

        let router = router
            .push(Router::new().get(index))
            .push(Router::with_path("health").get(health_check))
            .push(Router::with_path("metrics").get(metrics::report))
            .hoop(TrailingSlash::new(TrailingSlashAction::Remove));

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
            .push(Scalar::new("/api-docs/openapi.json").into_router("/scalar"));

        let cors = Self::build_cors(&self.config.cors);
        let ctx_injector = InjectContext::new(self.ctx.clone());

        let service = Service::new(router)
            .hoop(ctx_injector)
            .hoop(RateLimitMiddleware::new())
            .hoop(SecurityHeadersMiddleware::new())
            .hoop(MetricsMiddleware::new())
            .hoop(RequestId::new())
            .hoop(cors)
            .hoop(Timeout::new(Duration::from_secs(
                DEFAULT_REQUEST_TIMEOUT_SECS,
            )));

        // 创建监听器
        let listener = TcpListener::new(("0.0.0.0", port)).bind().await;
        tracing::info!("listening on 0.0.0.0:{}", port);
        tracing::info!("Swagger UI: http://localhost:{}/swagger-ui", port);
        tracing::info!("Scalar: http://localhost:{}/scalar", port);
        tracing::info!("Health check: http://localhost:{}/health", port);
        tracing::info!("Metrics: http://localhost:{}/metrics", port);

        let shutdown_flag = self.shutdown_flag.clone();
        let server = Server::new(listener);

        // 优雅关闭：让 server.serve 返回而不是 process::exit
        tokio::select! {
            _ = server.serve(service) => {
                tracing::info!("Server stopped");
            }
            _ = async {
                loop {
                    if shutdown_flag.load(Ordering::SeqCst) { break; }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                tracing::info!("Graceful shutdown in progress...");
                tokio::time::sleep(Duration::from_secs(GRACEFUL_SHUTDOWN_TIMEOUT_SECS)).await;
                tracing::info!("Graceful shutdown completed, exiting...");
                // 注意：Salvo 0.93 的 Server 没有 stop() 方法，
                // 此处从 select! 返回后，main 函数结束即进程退出
            } => {}
        }
        Ok(())
    }
}

#[endpoint]
async fn index(res: &mut Response) {
    crate::json_ok!(res, "Hello DaoYi Cloud Rust!");
}

#[endpoint]
async fn health_check(res: &mut Response) {
    let db_ok = db::get().ping().await.is_ok();
    let status = if db_ok { "UP" } else { "DOWN" };
    use salvo::writing::Json;
    res.render(Json(serde_json::json!({
        "status": status,
        "checks": { "database": status }
    })));
}
