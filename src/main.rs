pub mod logger;

use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();
    let router = Router::new().route("/", routing::get(index));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on http://0.0.0.0:3000");
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello Rust!"
}
