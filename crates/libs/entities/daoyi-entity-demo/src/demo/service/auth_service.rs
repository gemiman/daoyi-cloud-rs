use crate::demo::entity::prelude::*;
use crate::demo::entity::sys_user;
use crate::demo::models::auth::{LoginParams, LoginResult};
use daoyi_cloud_common::auth::jwt::{Principal, default_jwt};
use daoyi_cloud_common::constants::global_values::ROOT_ID;
use daoyi_cloud_common::db;
use daoyi_cloud_common::error::{ApiError, ApiResult};
use daoyi_cloud_common::utils::passwd_utils;
use sea_orm::prelude::*;

pub async fn login(params: LoginParams) -> ApiResult<LoginResult> {
    let model = SysUser::find()
        .filter(sys_user::Column::Account.eq(params.account))
        .one(db::get())
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码错误")))?;
    if !model.enabled {
        return Err(ApiError::Biz(String::from("账号已被禁用")));
    }
    if !passwd_utils::verify_passwd(&params.password, &model.password)? {
        return Err(ApiError::Biz(String::from("账号或密码错误")));
    }
    let principal = Principal {
        tenant_id: String::from(ROOT_ID),
        id: model.id,
        name: model.name,
    };
    let access_token = default_jwt().encode(principal)?;
    Ok(LoginResult { access_token })
}
