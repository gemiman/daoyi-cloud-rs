use crate::demo::entity::prelude::*;
use crate::demo::entity::sys_user;
use crate::demo::models::sys_user::{UserParams, UserQueryParams};
use daoyi_cloud_common::db;
use daoyi_cloud_common::error::{ApiError, ApiResult};
use daoyi_cloud_common::pojo::pagination::PageResult;
use daoyi_cloud_common::utils::passwd_utils::hash_passwd;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Condition, ExprTrait, IntoActiveModel, QueryOrder, QueryTrait};

pub async fn query_page(params: UserQueryParams) -> ApiResult<PageResult<sys_user::Model>> {
    let pagination = params.pagination();
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
        .paginate(dc, pagination.page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page_no - 1).await?;
    let result = PageResult::from_pagination(pagination, total, items);
    Ok(result)
}

pub async fn query_users() -> ApiResult<Vec<sys_user::Model>> {
    let dc = db::get();
    let users = SysUser::find()
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

pub async fn create_user(params: UserParams) -> ApiResult<sys_user::Model> {
    let mut active_model = params.into_active_model();
    active_model.password = ActiveValue::Set(hash_passwd(&active_model.password.take().unwrap())?);
    let model = active_model.insert(db::get()).await?;
    Ok(model)
}

pub async fn update_user_by_id(id: i64, params: UserParams) -> ApiResult<bool> {
    let model = SysUser::find_by_id(id)
        .one(db::get())
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("用户不存在")))?;
    let password = params.password.clone();
    let mut active_model = params.into_active_model();
    active_model.id = ActiveValue::Unchanged(model.id);
    if password.is_empty() {
        active_model.password = ActiveValue::Unchanged(model.password);
    } else {
        active_model.password = ActiveValue::Set(hash_passwd(&password)?);
    }
    active_model.update(db::get()).await?;
    Ok(true)
}

pub async fn get_user_by_id(id: i64) -> ApiResult<Option<sys_user::Model>> {
    let user = SysUser::find_by_id(id).one(db::get()).await?;
    Ok(user)
}

pub async fn delete_user_by_id(id: i64) -> ApiResult<bool> {
    let result = SysUser::delete_by_id(id).exec(db::get()).await?;
    tracing::info!("delete_user_by_id {id} result: {:?}", result);
    Ok(true)
}
