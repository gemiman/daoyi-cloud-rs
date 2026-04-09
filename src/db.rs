use crate::conf;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use std::cmp::max;
use std::time::Duration;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
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
    let cpus = num_cpus::get() as u32;
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
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
    Ok(dc)
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
