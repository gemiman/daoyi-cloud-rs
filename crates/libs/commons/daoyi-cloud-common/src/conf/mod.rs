pub mod auth;
pub mod db;
pub mod server;

use anyhow::Context;
pub use auth::AuthConfig;
use config::{Config, FileFormat};
pub use db::DatabaseConfig;
use serde::Deserialize;
pub use server::ServerConfig;
use std::path::PathBuf;
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
        // 配置文件路径：优先从环境变量 APP_CONFIG_PATH 读取
        let config_path = std::env::var("APP_CONFIG_PATH")
            .unwrap_or_else(|_| format!("resources/application-{app_name}"));

        let config_path = PathBuf::from(&config_path);
        let config_dir = config_path
            .parent()
            .unwrap_or(std::path::Path::new("resources"));
        let config_stem = config_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("application");

        let config = Config::builder()
            .add_source(
                config::File::with_name(&config_path.to_string_lossy())
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            // 支持 profile 覆盖：application-{profile}.yaml
            .add_source(
                config::File::with_name(config_dir.join(config_stem).to_string_lossy().as_ref())
                    .format(FileFormat::Yaml)
                    .required(false),
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
