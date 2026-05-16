use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    #[serde(default)]
    pub allowed_methods: Vec<String>,
    #[serde(default)]
    pub allowed_headers: Vec<String>,
    #[serde(default)]
    pub allow_credentials: bool,
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

/// TLS/HTTPS 配置
#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    /// 是否启用 HTTPS
    #[serde(default)]
    pub enabled: bool,
    /// 证书路径
    #[serde(default)]
    pub cert_path: String,
    /// 私钥路径
    #[serde(default)]
    pub key_path: String,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: String::new(),
            key_path: String::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
    #[serde(default)]
    pub cors: CorsConfig,
    /// TLS 配置
    #[serde(default)]
    pub tls: TlsConfig,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(8080)
    }
}
