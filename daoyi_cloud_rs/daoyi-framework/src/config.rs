use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkConfig {
    pub service_name: String,
    pub http_port: u16,
    pub nacos_endpoint: String,
    pub redis_url: String,
}

impl Default for FrameworkConfig {
    fn default() -> Self {
        Self {
            service_name: "daoyi-cloud-rs".to_string(),
            http_port: 18080,
            nacos_endpoint: "http://127.0.0.1:8848".to_string(),
            redis_url: "redis://127.0.0.1:6379".to_string(),
        }
    }
}
