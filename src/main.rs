pub mod api;
pub mod app;
pub mod conf;
pub mod db;
pub mod entity;
pub mod error;
pub mod logger;
pub mod response;
pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
