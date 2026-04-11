use crate::error::ApiResult;
use serde::Deserialize;
use wax::{Glob, Program};

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default)]
    ignore_urls: Vec<String>,
}

impl AuthConfig {
    pub fn ignored(&self, url: &str) -> ApiResult<bool> {
        path_any_matches(&self.ignore_urls, url)
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
