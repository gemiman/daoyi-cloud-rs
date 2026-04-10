pub mod latency;

use crate::app::AppState;
use crate::conf::ServerConfig;
use crate::error::{ApiError, ApiResult};
use crate::server::latency::LatencyOnResponse;
use axum::{Router, debug_handler, extract, routing};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);
        let port = self.config.port();
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
        tracing::info!("listening on {}://{}", "http", listener.local_addr()?);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &extract::Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        Router::new()
            .route("/", routing::get(index))
            .merge(router)
            .layer(tracing)
            .fallback(async |uri: extract::OriginalUri| -> ApiResult<()> {
                tracing::warn!(path = %uri.path(), "Not found");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async |req: extract::Request| -> ApiResult<()> {
                tracing::warn!(method = %req.method(), path = %req.uri().path(), "Method not allowed");
                Err(ApiError::MethodNotAllowed)
            })
            .with_state(state)
    }
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello DaoYi Cloud Rust!"
}
