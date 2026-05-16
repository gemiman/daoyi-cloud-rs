use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() {
    // 通过 RUST_LOG_FORMAT=json 切换生产环境 JSON 日志格式
    let use_json = std::env::var("RUST_LOG_FORMAT")
        .map(|v| v.eq_ignore_ascii_case("json"))
        .unwrap_or(false);

    let fmt_layer = if use_json {
        // JSON 格式日志（适合 ELK/Loki 日志聚合）
        tracing_subscriber::fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .boxed()
    } else {
        // 彩色文本格式日志（适合本地开发）
        tracing_subscriber::fmt::layer()
            .with_timer(ChronoLocal::new(String::from("%Y-%m-%d %H:%M:%S%.6f")))
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_target(false)
            .boxed()
    };

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt_layer)
        .init();

    if use_json {
        tracing::info!("JSON structured logging enabled");
    }
}
