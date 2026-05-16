use crate::{conf, context::AppContext, server};

use salvo::prelude::*;
use tokio::signal;

pub async fn run(app_name: &str, router: Router) -> anyhow::Result<()> {
    // === 构建 AppContext（完成所有初始化） ===
    let ctx = AppContext::build(app_name).await?;

    // === 创建 AppServer ===
    let srv = server::AppServer::new(conf::get().server(), ctx);

    // === 信号监听 ===
    let srv_for_signal = srv.clone();
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                tracing::info!("Received Ctrl+C, initiating graceful shutdown...");
                srv_for_signal.trigger_shutdown();
            }
            Err(e) => {
                tracing::error!("Failed to listen for Ctrl+C: {}", e);
            }
        }
    });

    // === 启动 HTTP 服务 ===
    srv.start(router).await
}
