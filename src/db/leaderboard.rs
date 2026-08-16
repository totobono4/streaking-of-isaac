use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Leaderboard {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub unit: String,
    pub stat: String,
    pub lower_is_better: bool,
    pub created_by: i64,
    pub created_at: String,
    pub updated_by: i64,
    pub updated_at: String,
}

pub async fn create_leaderboard(
    pool: &SqlitePool,
    slug: &str,
    title: &str,
    description: Option<&str>,
    unit: &str,
    stat: &str,
    lower_is_better: bool,
    created_by: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO leaderboards (slug, title, description, unit, stat, lower_is_better, created_by, updated_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(slug)
    .bind(title)
    .bind(description)
    .bind(unit)
    .bind(stat)
    .bind(lower_is_better)
    .bind(created_by)
    .bind(created_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_leaderboard(
    pool: &SqlitePool,
    id: i64,
    slug: &str,
    title: &str,
    description: Option<&str>,
    unit: &str,
    stat: &str,
    lower_is_better: bool,
    created_by: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE leaderboards SET slug = ?, title = ?, description = ?, unit = ?, stat = ?, lower_is_better = ?, updated_by = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(slug)
    .bind(title)
    .bind(description)
    .bind(unit)
    .bind(stat)
    .bind(lower_is_better)
    .bind(created_by)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_leaderboard(
    pool: &SqlitePool,
    id: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "DELETE FROM leaderboards WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_leaderboards(pool: &SqlitePool) -> anyhow::Result<Vec<Leaderboard>> {
    let rows = sqlx::query_as::<_, Leaderboard>(
        "SELECT id, slug, title, description, unit, stat, lower_is_better, created_by, created_at, updated_by, updated_at FROM leaderboards ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_leaderboard_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> anyhow::Result<Option<Leaderboard>> {
    let row = sqlx::query_as::<_, Leaderboard>(
        "SELECT id, slug, title, description, unit, stat, lower_is_better, created_by, created_at, updated_by, updated_at FROM leaderboards WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
