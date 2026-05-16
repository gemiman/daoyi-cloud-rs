pub use sea_orm_migration::prelude::*;

mod m20260516_000001_create_sys_user;
mod m20260516_000002_seed_sys_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260516_000001_create_sys_user::Migration),
            Box::new(m20260516_000002_seed_sys_user::Migration),
        ]
    }
}
