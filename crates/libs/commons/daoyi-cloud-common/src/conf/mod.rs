pub mod auth;
pub mod db;
pub mod server;

use anyhow::Context;
pub use auth::AuthConfig;
use config::{Config, FileFormat};
pub use db::DatabaseConfig;
use serde::Deserialize;
pub use server::ServerConfig;
use tokio::sync::OnceCell;

static CONFIG: OnceCell<AppConfig> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    auth: AuthConfig,
}

impl AppConfig {
    pub fn load(app_name: &str) -> anyhow::Result<()> {
        let config = Config::builder()
            .add_source(
                config::File::with_name(format!("resources/application-{app_name}").as_str())
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load conf"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize conf"))?;
        CONFIG
            .set(config)
            .with_context(|| anyhow::anyhow!("Failed to set conf"))?;
        Ok(())
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
}

pub fn get() -> &'static AppConfig {
    CONFIG
        .get()
        .unwrap_or_else(|| panic!("App config not initialized"))
}
