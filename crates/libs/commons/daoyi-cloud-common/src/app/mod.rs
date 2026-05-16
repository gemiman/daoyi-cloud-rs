use crate::{auth::jwt, conf, db, logger, server};

use salvo::prelude::*;
use tokio::signal;

pub async fn run(app_name: &str, router: Router) -> anyhow::Result<()> {
    // 1. 加载配置（支持自定义路径）
    conf::AppConfig::load(app_name)?;

    // 2. 初始化日志
    logger::init();
    tracing::info!("Starting app server...");

    // 3. 初始化雪花算法 ID
    crate::utils::id_utils::init()?;

    // 4. 初始化 RUST 配置中的 JWT（提前校验 secret 是否已配置）
    //    同时确保 JWT 在请求处理之前就可用
    jwt::init_from_config()?;

    // 5. 初始化数据库连接池
    db::init().await?;

    // 6. 创建服务
    let srv = server::AppServer::new(conf::get().server());

    // 7. 启动信号监听任务
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

    // 8. 启动服务
    srv.start(router).await
}
