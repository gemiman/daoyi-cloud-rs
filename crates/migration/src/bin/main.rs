use migration::Migrator;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:123456@127.0.0.1:3306/demo".to_string());

    let connect_options = sea_orm::ConnectOptions::new(&database_url);
    let db = sea_orm::Database::connect(connect_options).await.unwrap();

    Migrator::up(&db, None).await.unwrap();
    println!("Migration completed successfully!");
}
