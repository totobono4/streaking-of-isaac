use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
}

pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash: &str,
    is_admin: bool,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO users (username, password_hash, is_admin) VALUES (?, ?, ?)
         ON CONFLICT(username) DO UPDATE SET password_hash = excluded.password_hash, is_admin = 1",
    )
    .bind(username)
    .bind(password_hash)
    .bind(is_admin)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_user(
    pool: &SqlitePool,
    username: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        "DELETE FROM users WHERE username = ?",
    )
    .bind(username)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_users(
    pool: &SqlitePool,
) -> anyhow::Result<Vec<User>> {
    let rows = sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn find_user_by_username(
    pool: &SqlitePool,
    username: &str,
) -> anyhow::Result<Option<User>> {
    let row = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, is_admin FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn user_count(
    pool: &SqlitePool,
) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users",
    )
    .fetch_one(pool)
    .await?;
    Ok(count)
}
