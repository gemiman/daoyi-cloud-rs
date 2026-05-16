use serde::Deserialize;

/// 数据库连接池配置
#[derive(Debug, Deserialize, Clone)]
pub struct DbPoolConfig {
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout_secs: u64,
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout_secs: u64,
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_secs: u64,
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime_secs: u64,
    #[serde(default)]
    pub sqlx_logging: bool,
}

impl Default for DbPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: default_min_connections(),
            max_connections: default_max_connections(),
            connect_timeout_secs: default_connect_timeout(),
            acquire_timeout_secs: default_acquire_timeout(),
            idle_timeout_secs: default_idle_timeout(),
            max_lifetime_secs: default_max_lifetime(),
            sqlx_logging: false,
        }
    }
}

fn default_min_connections() -> u32 {
    2
}
fn default_max_connections() -> u32 {
    10
}
fn default_connect_timeout() -> u64 {
    30
}
fn default_acquire_timeout() -> u64 {
    30
}
fn default_idle_timeout() -> u64 {
    60
}
fn default_max_lifetime() -> u64 {
    300
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    /// 连接池配置（可选）
    #[serde(default)]
    pub pool: DbPoolConfig,
}

impl DatabaseConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("localhost")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3306)
    }

    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("root")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("root")
    }

    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("daoyi_cloud")
    }
}
