use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = sea_orm::prelude::DateTime::default();
        let insert = Query::insert()
            .into_table(crate::m20260516_000001_create_sys_user::SysUser::Table)
            .columns([
                crate::m20260516_000001_create_sys_user::SysUser::Id,
                crate::m20260516_000001_create_sys_user::SysUser::Name,
                crate::m20260516_000001_create_sys_user::SysUser::Gender,
                crate::m20260516_000001_create_sys_user::SysUser::Account,
                crate::m20260516_000001_create_sys_user::SysUser::Password,
                crate::m20260516_000001_create_sys_user::SysUser::MobilePhone,
                crate::m20260516_000001_create_sys_user::SysUser::Birthday,
                crate::m20260516_000001_create_sys_user::SysUser::Enabled,
                crate::m20260516_000001_create_sys_user::SysUser::CreatedAt,
                crate::m20260516_000001_create_sys_user::SysUser::UpdatedAt,
            ])
            .values_panic([
                1i64.into(),
                "张三".into(),
                "male".into(),
                "admin".into(),
                "$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK".into(),
                "+8613800138000".into(),
                sea_orm::prelude::Date::from_ymd_opt(1995, 5, 18)
                    .unwrap()
                    .into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                2i64.into(),
                "李四".into(),
                "female".into(),
                "lisi".into(),
                "$2b$12$PsumwxjxX/o1RNOKpkc.Kuxea0izqSuhaod4PCudXoRh3zet1TASK".into(),
                "+8613900940724".into(),
                sea_orm::prelude::Date::from_ymd_opt(1998, 7, 22)
                    .unwrap()
                    .into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(crate::m20260516_000001_create_sys_user::SysUser::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
