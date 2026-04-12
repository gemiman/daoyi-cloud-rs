use crate::conf;
use crate::utils::id_utils;
use crate::{db, logger, server};
use salvo::prelude::*;

pub async fn run(app_name: &str, router: Router) -> anyhow::Result<()> {
    conf::AppConfig::load(app_name)?;
    logger::init();
    tracing::info!("Starting app server...");
    id_utils::init()?;

    db::init().await?;

    let srv = server::AppServer::new(conf::get().server());
    srv.start(router).await
}
