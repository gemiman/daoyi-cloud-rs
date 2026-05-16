use std::sync::Arc;

use crate::auth::jwt::{self, JWT};
use crate::conf;
use crate::db;
use sea_orm::DatabaseConnection;

const CTX_DEPOT_KEY: &str = "daoyi_app_context";

/// 应用上下文：持有 DB 连接和 JWT 实例的快照，通过自定义中间件注入 Depo。
///
/// # 注入方式
///
/// ```ignore
/// // 启动时注入
/// Service::new(router).hoop(InjectContext::new(ctx));
///
/// // handler 中提取
/// let ctx = AppContext::from_depot(depot).unwrap();
/// ```
pub struct AppContext {
    pub db: DatabaseConnection,
    pub jwt: JWT,
}

impl AppContext {
    /// 完整构建（初始化 + 创建上下文）
    pub async fn build(app_name: &str) -> anyhow::Result<Arc<Self>> {
        let _ = dotenvy::dotenv();
        crate::logger::init();
        tracing::info!("Starting app server...");

        conf::AppConfig::load(app_name)?;
        crate::utils::id_utils::init()?;
        jwt::init_from_config()?;
        db::init().await?;

        let jwt = jwt::default_jwt().clone();
        let db = db::get().clone();
        Ok(Arc::new(Self { db, jwt }))
    }

    /// 从 Salvo Depot 中提取 AppContext
    pub fn from_depot(depot: &Depot) -> Option<&Arc<Self>> {
        depot.get::<Arc<AppContext>>(CTX_DEPOT_KEY).ok()
    }
}

// ---------------------------------------------------------------------------
// 注入中间件
// ---------------------------------------------------------------------------
use salvo::prelude::*;

/// 将 Arc<AppContext> 注入 Salvo Depot 的中间件
pub struct InjectContext {
    ctx: Arc<AppContext>,
}

impl InjectContext {
    pub fn new(ctx: Arc<AppContext>) -> Self {
        Self { ctx }
    }
}

#[handler]
impl InjectContext {
    async fn handle(
        &self,
        _req: &mut Request,
        depot: &mut Depot,
        _res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        depot.insert(CTX_DEPOT_KEY, self.ctx.clone());
        ctrl.call_next(_req, depot, _res).await;
    }
}
