//! Shared metadata for the Rust rewrite; will later hold cross-cutting helpers.

pub const PROJECT_NAME: &str = "daoyi-cloud-rs";

pub fn banner() -> String {
    format!("{PROJECT_NAME} - Rust rewrite skeleton")
}
