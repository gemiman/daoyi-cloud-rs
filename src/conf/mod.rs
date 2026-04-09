pub mod server;

use anyhow::Context;
use config::{Config, FileFormat};
use serde::Deserialize;
pub use server::ServerConfig;
use std::sync::LazyLock;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initialize conf"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("resources/application")
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
            .with_context(|| anyhow::anyhow!("Failed to deserialize conf"))
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
