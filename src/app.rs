use crate::{conf, db, logger, server};
use axum::Router;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub dc: DatabaseConnection,
}

impl AppState {
    pub fn new(dc: DatabaseConnection) -> Self {
        Self { dc }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("Starting app server...");

    let dc = db::init().await?;
    let state = AppState::new(dc);
    let server = server::Server::new(&conf::get().server());
    server.start(state, router).await
}
