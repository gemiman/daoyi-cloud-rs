use crate::app::AppState;
use crate::entity::prelude::*;
use crate::entity::sys_user;
use axum::extract::State;
use axum::{Router, debug_handler, routing};
use sea_orm::Condition;

use crate::response::CommonResult;
use crate::success;
use sea_orm::prelude::*;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", routing::get(query_users))
}

#[debug_handler]
async fn query_users(
    State(AppState { dc }): State<AppState>,
) -> CommonResult<Vec<sys_user::Model>> {
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
    success!(users)
}
