pub mod app;
pub mod conf;
pub mod db;
pub mod entity;
pub mod logger;
pub mod server;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, debug_handler, routing};
use sea_orm::Condition;

use crate::app::AppState;
use crate::entity::sys_user;
use entity::prelude::*;
use sea_orm::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", routing::get(index))
        .route("/users", routing::get(query_users));
    app::run(router).await
}

#[debug_handler]
async fn query_users(State(AppState { dc }): State<AppState>) -> impl IntoResponse {
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
