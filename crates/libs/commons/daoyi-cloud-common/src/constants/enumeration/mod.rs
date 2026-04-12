use salvo::oapi::ToSchema;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

/// 性别
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum, ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Gender {
    /// 男
    Male,
    /// 女
    Female,
}
