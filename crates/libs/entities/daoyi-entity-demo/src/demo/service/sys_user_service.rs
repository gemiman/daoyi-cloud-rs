use crate::demo::entity::prelude::*;
use crate::demo::entity::sys_user;
use crate::demo::models::sys_user::{UserParams, UserQueryParams};
use daoyi_cloud_common::error::{ApiError, ApiResult};
use daoyi_cloud_common::pojo::pagination::PageResult;
use daoyi_cloud_common::utils::passwd_utils::hash_passwd;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Condition, ExprTrait, IntoActiveModel, QueryOrder, QueryTrait};

/// 分页查询（DI 版本：接收 db 连接）
pub async fn query_page(
    db: &DatabaseConnection,
    params: UserQueryParams,
) -> ApiResult<PageResult<sys_user::Model>> {
    let pagination = params.pagination();
    let paginator = SysUser::find()
        .apply_if(params.keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .and(sys_user::Column::Name.contains(keyword))
                    .or(sys_user::Column::Account.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(db, pagination.page_size);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(pagination.page_no - 1).await?;
    Ok(PageResult::from_pagination(pagination, total, items))
}

/// 复杂条件查询（示例）
pub async fn query_users(db: &DatabaseConnection) -> ApiResult<Vec<sys_user::Model>> {
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
        .all(db)
        .await?;
    Ok(users)
}

/// 创建用户
pub async fn create_user(
    db: &DatabaseConnection,
    params: UserParams,
) -> ApiResult<sys_user::Model> {
    let mut active_model = params.into_active_model();
    active_model.password = ActiveValue::Set(hash_passwd(&active_model.password.take().unwrap())?);
    let model = active_model.insert(db).await?;
    Ok(model)
}

/// 更新用户
pub async fn update_user_by_id(
    db: &DatabaseConnection,
    id: i64,
    params: UserParams,
) -> ApiResult<bool> {
    let model = SysUser::find_by_id(id)
        .one(db)
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
    active_model.update(db).await?;
    Ok(true)
}

/// 按 ID 查询
pub async fn get_user_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> ApiResult<Option<sys_user::Model>> {
    let user = SysUser::find_by_id(id).one(db).await?;
    Ok(user)
}

/// 删除用户
pub async fn delete_user_by_id(db: &DatabaseConnection, id: i64) -> ApiResult<bool> {
    let result = SysUser::delete_by_id(id).exec(db).await?;
    tracing::info!("delete_user_by_id {id} result: {:?}", result);
    Ok(true)
}
