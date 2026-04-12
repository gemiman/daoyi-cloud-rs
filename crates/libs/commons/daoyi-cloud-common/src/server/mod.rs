pub mod latency;

use crate::app::AppState;
use crate::conf::ServerConfig;
use crate::error::{ApiError, ApiResult};
use crate::response::CommonResult;
use crate::server::latency::LatencyOnResponse;
use crate::success;
use crate::utils::id_utils;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::{Router, debug_handler, extract, routing};
use bytesize::ByteSize;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
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
        let timeout =
            TimeoutLayer::with_status_code(StatusCode::GATEWAY_TIMEOUT, Duration::from_secs(120));
        let body_limit = DefaultBodyLimit::max(ByteSize::gib(1).as_u64() as usize);
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_methods(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_hours(12));
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &extract::Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = id_utils::xid();
                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        let normalize_path = NormalizePathLayer::trim_trailing_slash();
        Router::new()
            .route("/", routing::get(index))
            .merge(router)
            .layer(timeout)
            .layer(body_limit)
            .layer(normalize_path)
            .layer(tracing)
            .fallback(async |uri: extract::OriginalUri| -> ApiResult<()> {
                tracing::warn!(path = %uri.path(), "Not found");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async |req: extract::Request| -> ApiResult<()> {
                tracing::warn!(method = %req.method(), path = %req.uri().path(), "Method not allowed");
                Err(ApiError::MethodNotAllowed)
            })
            .layer(cors)
            .with_state(state)
    }
}

#[debug_handler]
async fn index() -> CommonResult<&'static str> {
    success!("Hello DaoYi Cloud Rust!")
}
