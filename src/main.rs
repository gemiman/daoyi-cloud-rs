pub mod conf;
pub mod db;
pub mod entity;
pub mod logger;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, debug_handler, routing};
use sea_orm::Condition;
use tokio::net::TcpListener;

use crate::entity::sys_user;
use entity::prelude::*;
use sea_orm::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let dc = db::init().await?;
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(query_users))
        .with_state(dc);
    let port = conf::get().server().port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}

#[debug_handler]
async fn query_users(State(dc): State<DatabaseConnection>) -> impl IntoResponse {
    let users = SysUser::find()
        // .filter(sys_user::Column::Gender.eq("male"))
        .filter(
            Condition::all()
                .add(sys_user::Column::Gender.eq("male"))
                .add(sys_user::Column::Name.starts_with("张"))
                .add(
                    Condition::any()
                        .add(sys_user::Column::Name.eq("张三"))
                        .add(sys_user::Column::Name.contains("张三丰")),
                ),
        )
        .all(&dc)
        .await
        .unwrap();
    axum::Json(users)
}

#[debug_handler]
async fn index() -> &'static str {
    "Hello Rust!"
}
