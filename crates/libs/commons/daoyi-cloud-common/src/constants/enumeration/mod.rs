use sea_orm::prelude::*;
// use sea_orm::{ActiveValue, IntoActiveValue};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
#[schema(title = "Gender", description = "性别")]
pub enum Gender {
    /// 男
    // #[sea_orm(string_value = "01")]
    // #[serde(rename = "01")]
    Male,
    /// 女
    // #[sea_orm(string_value = "02")]
    // #[serde(rename = "02")]
    Female,
}

// impl IntoActiveValue<Gender> for Gender {
//     fn into_active_value(self) -> ActiveValue<Gender> {
//         ActiveValue::Set(self)
//     }
// }
