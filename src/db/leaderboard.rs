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
        "INSERT INTO leaderboards (slug, title, description, unit, stat, lower_is_better, created_by) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(slug)
    .bind(title)
    .bind(description)
    .bind(unit)
    .bind(stat)
    .bind(lower_is_better)
    .bind(created_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_leaderboards(pool: &SqlitePool) -> anyhow::Result<Vec<Leaderboard>> {
    let rows = sqlx::query_as::<_, Leaderboard>(
        "SELECT id, slug, title, description, unit, stat, lower_is_better FROM leaderboards ORDER BY created_at DESC",
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
        "SELECT id, slug, title, description, unit, stat, lower_is_better FROM leaderboards WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
