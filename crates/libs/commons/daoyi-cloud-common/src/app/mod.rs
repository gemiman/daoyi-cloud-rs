use crate::{auth::jwt, conf, db, logger, server};

use salvo::prelude::*;
use tokio::signal;

pub async fn run(app_name: &str, router: Router) -> anyhow::Result<()> {
    // 1. 加载 .env 文件（可选，用于本地开发环境变量）
    let _ = dotenvy::dotenv();

    // 2. 加载配置（支持自定义路径）
    conf::AppConfig::load(app_name)?;

    // 3. 初始化日志
    logger::init();
    tracing::info!("Starting app server...");

    // 4. 初始化雪花算法 ID
    crate::utils::id_utils::init()?;

    // 5. 初始化 JWT（启动时校验 secret 是否已配置）
    jwt::init_from_config()?;

    // 6. 初始化数据库连接池
    db::init().await?;

    // 7. 创建服务
    let srv = server::AppServer::new(conf::get().server());

    // 8. 启动信号监听任务
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

    // 9. 启动服务
    srv.start(router).await
}
