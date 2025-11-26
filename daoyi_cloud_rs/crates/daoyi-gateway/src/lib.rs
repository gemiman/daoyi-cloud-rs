use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use salvo::cors::{AllowedHeaders, AllowOrigin, Cors};
use salvo::logging::Logger;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// 监听地址，默认 0.0.0.0:8080
    pub listen: String,
    /// 允许的跨域来源，默认 *
    pub allow_origins: Vec<String>,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            listen: "0.0.0.0:8080".into(),
            allow_origins: vec!["*".into()],
        }
    }
}

/// 加载网关配置：config-example.toml -> config.toml -> 环境变量（前缀 DAOGW_）
pub fn load_gateway_config() -> GatewayConfig {
    Figment::from(Serialized::defaults(GatewayConfig::default()))
        .merge(Toml::file("config-example.toml").nested())
        .merge(Toml::file("config.toml").nested())
        .merge(Env::prefixed("DAOGW_").split("__"))
        .extract()
        .unwrap_or_default()
}

fn build_router() -> Router {
    Router::new()
        .hoop(Logger::new())
        .hoop(RequestId::new())
        .hoop(build_cors())
        .push(Router::with_path("health").get(health))
        .push(Router::with_path("<**rest>").any(fallback_not_found))
}

fn build_cors() -> Cors {
    // 允许自定义 origins，* 时使用 AllowOrigin::any
    let cfg = load_gateway_config();
    let origins = if cfg.allow_origins.iter().any(|o| o == "*") {
        AllowOrigin::any()
    } else {
        AllowOrigin::list(cfg.allow_origins)
    };
    Cors::new()
        .allow_origin(origins)
        .allow_headers(AllowedHeaders::any())
        .allow_methods(Any)
        .allow_credentials(true)
}

#[handler]
async fn health(res: &mut Response) {
    res.render(Text::Plain("ok"));
}

#[handler]
async fn fallback_not_found(req: &mut Request, res: &mut Response) {
    let path = req.uri().path().to_owned();
    res.status_code(StatusCode::NOT_FOUND);
    res.render(Text::Json(format!(
        "{{\"code\":404,\"message\":\"gateway route not found: {}\"}}",
        path
    )));
}

pub async fn run_gateway() -> anyhow::Result<()> {
    let cfg = load_gateway_config();
    if tracing_subscriber::registry().has_default() {
        // already inited by caller
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
            .json()
            .init();
    }

    tracing::info!(target = "daoyi-gateway", listen = %cfg.listen, "starting gateway");
    let router = build_router();
    let acceptor = TcpListener::bind(&cfg.listen).bind().await?;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
