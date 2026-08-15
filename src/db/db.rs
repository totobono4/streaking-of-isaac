use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

pub async fn init_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        Sqlite::create_database(database_url).await?;
    }
    let pool = SqlitePool::connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
