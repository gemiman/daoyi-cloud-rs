use crate::demo::entity::prelude::*;
use crate::demo::entity::sys_user;
use crate::demo::models::sys_user::UserQueryParams;
use daoyi_cloud_common::db;
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::pojo::pagination::PageResult;
use sea_orm::prelude::*;
use sea_orm::{Condition, ExprTrait, QueryOrder, QueryTrait};

pub async fn query_page(params: UserQueryParams) -> ApiResult<PageResult<sys_user::Model>> {
    let dc = db::get();
    let paginator = SysUser::find()
        .apply_if(params.keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .and(sys_user::Column::Name.contains(keyword))
                    .or(sys_user::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(dc, params.pagination.page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(params.pagination.page_no - 1).await?;
    let result = PageResult::from_pagination(params.pagination, total, items);
    Ok(result)
}

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
