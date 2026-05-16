use crate::error::ApiResult;
use serde::Deserialize;
use wax::{Glob, Program};

use crate::constants::default_values;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    ignore_urls: Vec<String>,
    #[serde(default)]
    pub jwt: JwtSectionConfig,
}

#[derive(Debug, Deserialize)]
pub struct JwtSectionConfig {
    #[serde(default)]
    pub secret: String,
    #[serde(default = "default_jwt_expiration")]
    pub expiration_secs: u64,
    #[serde(default = "default_jwt_audience")]
    pub audience: String,
    #[serde(default = "default_jwt_issuer")]
    pub issuer: String,
}

impl Default for JwtSectionConfig {
    fn default() -> Self {
        Self {
            secret: String::new(),
            expiration_secs: default_values::DEFAULT_JWT_EXPIRATION_SECS,
            audience: default_values::DEFAULT_JWT_AUDIENCE.to_string(),
            issuer: default_values::DEFAULT_JWT_ISSUER.to_string(),
        }
    }
}

fn default_jwt_expiration() -> u64 {
    default_values::DEFAULT_JWT_EXPIRATION_SECS
}

fn default_jwt_audience() -> String {
    default_values::DEFAULT_JWT_AUDIENCE.to_string()
}

fn default_jwt_issuer() -> String {
    default_values::DEFAULT_JWT_ISSUER.to_string()
}

impl AuthConfig {
    pub fn ignored(&self, url: &str) -> ApiResult<bool> {
        path_any_matches(&self.ignore_urls, url)
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            ignore_urls: vec![],
            jwt: JwtSectionConfig::default(),
        }
    }
}

fn path_matches(pattern: &str, path: &str) -> ApiResult<bool> {
    let glob = Glob::new(pattern)?;
    Ok(glob.is_match(path))
}

fn path_any_matches<A: AsRef<str>>(patterns: &[A], path: &str) -> ApiResult<bool> {
    for pattern in patterns {
        if path_matches(pattern.as_ref(), path)? {
            return Ok(true);
        }
    }
    Ok(false)
}
