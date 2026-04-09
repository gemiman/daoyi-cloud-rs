pub mod conf;
pub mod db;
pub mod logger;

use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let _dc = db::init().await?;
    let router = Router::new().route("/", routing::get(index));
    let port = conf::get().server().port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello Rust!"
}
