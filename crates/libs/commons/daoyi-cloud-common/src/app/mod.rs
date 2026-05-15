use crate::conf;
use crate::utils::id_utils;
use crate::{db, logger, server};
use salvo::prelude::*;
use tokio::signal;

pub async fn run(app_name: &str, router: Router) -> anyhow::Result<()> {
    conf::AppConfig::load(app_name)?;
    logger::init();
    tracing::info!("Starting app server...");
    id_utils::init()?;

    db::init().await?;

    let srv = server::AppServer::new(conf::get().server());

    // 启动信号监听任务
    let srv_for_signal = srv.clone();
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                tracing::info!("Received Ctrl+C, initiating graceful shutdown...");
                // 通知所有服务开始优雅关闭
                srv_for_signal.trigger_shutdown();
            }
            Err(e) => {
                tracing::error!("Failed to listen for Ctrl+C: {}", e);
            }
        }
    });

    // 启动服务
    srv.start(router).await
}
