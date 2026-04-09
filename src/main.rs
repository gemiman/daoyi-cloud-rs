pub mod conf;
pub mod logger;

use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init();
    let router = Router::new().route("/", routing::get(index));
    let port = conf::get().server.port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello Rust!"
}
