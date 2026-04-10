use crate::conf::AppConfig;
use crate::utils::id_utils;
use crate::{conf, db, logger, server};
use axum::Router;

#[derive(Clone)]
pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

pub async fn run(app_name: &str, router: Router<AppState>) -> anyhow::Result<()> {
    AppConfig::load(app_name)?;
    logger::init();
    tracing::info!("Starting app server...");
    id_utils::init()?;

    db::init().await?;

    let state = AppState::new();
    let server = server::Server::new(&conf::get().server());
    server.start(state, router).await
}
