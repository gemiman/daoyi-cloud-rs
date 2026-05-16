pub mod latency;

use crate::conf::ServerConfig;
use crate::db;
use salvo::cors::{Any, Cors};
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
    shutdown_flag: Arc<AtomicBool>,
}

impl Clone for AppServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config,
            shutdown_flag: self.shutdown_flag.clone(),
        }
    }
}

impl AppServer {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self {
            config,
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 触发优雅关闭
    pub fn trigger_shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);
        tracing::info!("Shutdown signal received, initiating graceful shutdown...");
    }

    pub async fn start(&self, router: Router) -> anyhow::Result<()> {
        let port = self.config.port();

        let router = router
            .push(Router::new().get(index))
            .push(Router::with_path("health").get(health_check))
            .hoop(TrailingSlash::new(TrailingSlashAction::Remove));

        // 创建 OpenAPI 文档（必须在所有路由注册之后 merge）
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

        // CORS
        let cors = Cors::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(43200))
            .into_handler();

        // 全局中间件
        let service = Service::new(router)
            .hoop(RequestId::new())
            .hoop(cors)
            .hoop(Timeout::new(Duration::from_secs(
                DEFAULT_REQUEST_TIMEOUT_SECS,
            )));

        let listener = TcpListener::new(("0.0.0.0", port)).bind().await;
        tracing::info!("listening on http://0.0.0.0:{}", port);
        tracing::info!("Swagger UI: http://localhost:{}/swagger-ui", port);
        tracing::info!("Scalar: http://localhost:{}/scalar", port);
        tracing::info!("Health check: http://localhost:{}/health", port);

        let shutdown_flag = self.shutdown_flag.clone();
        let server = Server::new(listener);

        tokio::select! {
            _ = server.serve(service) => {
                tracing::info!("Server stopped");
            }
            _ = async {
                loop {
                    if shutdown_flag.load(Ordering::SeqCst) {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                tracing::info!("Graceful shutdown in progress...");
                tokio::time::sleep(Duration::from_secs(GRACEFUL_SHUTDOWN_TIMEOUT_SECS)).await;
                tracing::info!("Graceful shutdown completed, exiting...");
                std::process::exit(0);
            } => {}
        }

        Ok(())
    }
}

/// 首页响应
#[endpoint]
async fn index(res: &mut Response) {
    crate::json_ok!(res, "Hello DaoYi Cloud Rust!");
}

/// 健康检查端点
#[endpoint]
async fn health_check(res: &mut Response) {
    let db_ok = db::get().ping().await.is_ok();
    let status = if db_ok { "UP" } else { "DOWN" };

    use salvo::writing::Json;
    res.render(Json(serde_json::json!({
        "status": status,
        "checks": {
            "database": status
        }
    })));
}
