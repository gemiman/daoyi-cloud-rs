use crate::conf;
use anyhow::Context;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use std::time::Duration;
use tokio::sync::OnceCell;

static DB_CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub fn get() -> &'static DatabaseConnection {
    DB_CONN
        .get()
        .unwrap_or_else(|| panic!("Database connection not initialized"))
}

pub async fn init() -> anyhow::Result<()> {
    let db_conf = conf::get().database();
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_conf.user(),
        db_conf.password(),
        db_conf.host(),
        db_conf.port(),
        db_conf.database()
    );
    tracing::info!("Connecting to database: {}", url);

    let pool = &db_conf.pool;
    let mut options = ConnectOptions::new(url);
    options
        .min_connections(pool.min_connections)
        .max_connections(pool.max_connections)
        .connect_timeout(Duration::from_secs(pool.connect_timeout_secs))
        .acquire_timeout(Duration::from_secs(pool.acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(pool.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(pool.max_lifetime_secs))
        .sqlx_logging(pool.sqlx_logging);

    let dc = Database::connect(options).await?;
    dc.ping().await?;
    tracing::info!("Database connection established");
    log_database_version(&dc).await?;
    DB_CONN
        .set(dc)
        .with_context(|| "Failed to set database connection")?;
    Ok(())
}

async fn log_database_version(dc: &DatabaseConnection) -> anyhow::Result<()> {
    let result = dc
        .query_one_raw(Statement::from_string(
            dc.get_database_backend(),
            String::from("SELECT VERSION()"),
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get database version"))?;
    let version: String = result.try_get_by_index(0)?;
    tracing::info!("Database version: {}", version);
    Ok(())
}
