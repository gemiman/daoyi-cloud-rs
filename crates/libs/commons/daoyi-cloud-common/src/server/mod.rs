pub mod latency;

use crate::conf::ServerConfig;
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

        let shutdown_flag = self.shutdown_flag.clone();
        let server = Server::new(listener);

        // 使用 select! 同时监听服务运行和关闭信号
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
                // 给一个时间窗口让剩余请求完成
                tokio::time::sleep(Duration::from_secs(GRACEFUL_SHUTDOWN_TIMEOUT_SECS)).await;
                tracing::info!("Graceful shutdown completed, exiting...");
                // 强制退出进程
                std::process::exit(0);
            } => {}
        }

        Ok(())
    }
}

#[handler]
async fn index(res: &mut Response) {
    crate::success!(res, "Hello DaoYi Cloud Rust!");
}
