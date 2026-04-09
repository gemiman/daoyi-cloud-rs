use crate::conf;
// use std::cmp::max;
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
        "postgres://{}:{}@{}:{}/{}",
        db_conf.user(),
        db_conf.password(),
        db_conf.host(),
        db_conf.port(),
        db_conf.database()
    );
    tracing::info!("Connecting to database: {}", url);
    let mut options = ConnectOptions::new(url);
    // let cpus = num_cpus::get() as u32;
    options
        // .min_connections(max(cpus * 4, 10))
        // .max_connections(max(cpus * 8, 20))
        .min_connections(2)
        .max_connections(10)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(db_conf.schema());
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
    let version = dc
        .query_one_raw(Statement::from_string(
            dc.get_database_backend(),
            String::from("SELECT version()"),
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get database version"))?;
    tracing::info!(
        "Database version: {}",
        version.try_get_by_index::<String>(0)?
    );
    Ok(())
}
