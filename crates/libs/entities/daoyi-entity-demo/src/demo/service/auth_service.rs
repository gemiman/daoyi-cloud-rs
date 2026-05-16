use crate::demo::entity::prelude::*;
use crate::demo::entity::sys_user;
use crate::demo::models::auth::{LoginParams, LoginResult};
use daoyi_cloud_common::auth::jwt::{Principal, default_jwt};
use daoyi_cloud_common::constants::global_values::ROOT_ID;
use daoyi_cloud_common::error::{ApiError, ApiResult};
use daoyi_cloud_common::utils::passwd_utils;
use sea_orm::prelude::*;

/// 用户登录（DI 版本：接收 db 连接，不再使用全局 `db::get()`）
pub async fn login(db: &DatabaseConnection, params: LoginParams) -> ApiResult<LoginResult> {
    let model = SysUser::find()
        .filter(sys_user::Column::Account.eq(params.account))
        .one(db)
        .await?
        .ok_or_else(|| ApiError::Biz(String::from("账号或密码错误")))?;
    if !model.enabled {
        return Err(ApiError::Biz(String::from("账号已被禁用")));
    }
    if !passwd_utils::verify_passwd(&params.password, &model.password)? {
        return Err(ApiError::Biz(String::from("账号或密码错误")));
    }
    let principal = Principal {
        tenant_id: ROOT_ID,
        id: model.id,
        name: model.name,
        roles: vec!["user".to_string()],
        permissions: vec!["user:read".to_string(), "user:write".to_string()],
    };
    let access_token = default_jwt().encode(principal)?;
    Ok(LoginResult { access_token })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_login_invalid_password() {
        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::MySql)
            .append_query_results([vec![sys_user::Model {
                id: 1,
                name: "测试".into(),
                gender: daoyi_cloud_common::constants::enumeration::Gender::Male,
                account: "test".into(),
                password: "$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK".into(),
                mobile_phone: "+8613912345678".into(),
                birthday: sea_orm::prelude::Date::from_ymd_opt(2000, 1, 1).unwrap(),
                enabled: true,
                created_at: sea_orm::prelude::DateTime::default(),
                updated_at: sea_orm::prelude::DateTime::default(),
            }]])
            .into_connection();

        let result = login(
            &db,
            LoginParams {
                account: "test".into(),
                password: "wrong_password".into(),
            },
        )
        .await;

        assert!(result.is_err());
        match result {
            Err(ApiError::Biz(msg)) => assert!(msg.contains("密码错误") || msg.contains("账号")),
            _ => panic!("expected Biz error"),
        }
    }

    #[tokio::test]
    async fn test_login_disabled_user() {
        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::MySql)
            .append_query_results([vec![sys_user::Model {
                id: 2,
                name: "禁用用户".into(),
                gender: daoyi_cloud_common::constants::enumeration::Gender::Female,
                account: "disabled".into(),
                password: "hash".into(),
                mobile_phone: "+8613912345678".into(),
                birthday: sea_orm::prelude::Date::from_ymd_opt(2000, 1, 1).unwrap(),
                enabled: false,
                created_at: sea_orm::prelude::DateTime::default(),
                updated_at: sea_orm::prelude::DateTime::default(),
            }]])
            .into_connection();

        let result = login(
            &db,
            LoginParams {
                account: "disabled".into(),
                password: "any_password".into(),
            },
        )
        .await;

        assert!(result.is_err());
        match result {
            Err(ApiError::Biz(msg)) => assert_eq!(msg, "账号已被禁用"),
            _ => panic!("expected Biz error"),
        }
    }
}
