use tracing_subscriber::EnvFilter;

pub fn init_tracing(service_name: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .try_init()
        .ok();
    tracing::info!(service = service_name, "tracing initialized");
}
