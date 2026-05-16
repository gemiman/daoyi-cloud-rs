use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SysUser::Table)
                    .if_not_exists()
                    .col(pk_auto(SysUser::Id))
                    .col(string_len(SysUser::Name, 16).not_null())
                    .col(string_len(SysUser::Gender, 8).not_null())
                    .col(string_len(SysUser::Account, 16).not_null())
                    .col(string_len(SysUser::Password, 64).not_null())
                    .col(string_len(SysUser::MobilePhone, 16).not_null())
                    .col(date(SysUser::Birthday).not_null())
                    .col(boolean(SysUser::Enabled).not_null().default(true))
                    .col(
                        date_time(SysUser::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        date_time(SysUser::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SysUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SysUser {
    Table,
    Id,
    Name,
    Gender,
    Account,
    Password,
    MobilePhone,
    Birthday,
    Enabled,
    CreatedAt,
    UpdatedAt,
}
