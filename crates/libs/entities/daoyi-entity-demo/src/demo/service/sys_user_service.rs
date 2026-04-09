use crate::demo::entity::prelude::*;
use crate::demo::entity::sys_user;
use daoyi_cloud_common::db;
use daoyi_cloud_common::error::ApiResult;
use sea_orm::Condition;
use sea_orm::prelude::*;

pub async fn query_users() -> ApiResult<Vec<sys_user::Model>> {
    let dc = db::get();
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
        .all(dc)
        .await?;
    Ok(users)
}
