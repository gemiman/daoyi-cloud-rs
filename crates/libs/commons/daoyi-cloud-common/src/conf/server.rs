use serde::Deserialize;

/// CORS 配置
#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    /// 允许的 Origin，空表示允许所有
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    /// 允许的 HTTP 方法，空表示允许所有
    #[serde(default)]
    pub allowed_methods: Vec<String>,
    /// 允许的请求头，空表示允许所有
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    /// 是否允许携带凭证（cookies）
    #[serde(default)]
    pub allow_credentials: bool,
    /// 预检请求缓存时间（秒）
    #[serde(default = "default_max_age")]
    pub max_age_secs: u64,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec![],
            allowed_methods: vec![],
            allowed_headers: vec![],
            allow_credentials: false,
            max_age_secs: default_max_age(),
        }
    }
}

fn default_max_age() -> u64 {
    43200
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
    /// CORS 配置（可选）
    #[serde(default)]
    pub cors: CorsConfig,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }
}
