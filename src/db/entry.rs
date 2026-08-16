use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Entry {
    pub id: i64,
    pub leaderboard_id: i64,
    pub player_name: String,
    pub player_link: Option<String>,
    pub score: i64,
    pub stat_text: String,
    pub stat_link: Option<String>,
    pub note: String,
    pub created_by: i64,
    pub created_at: String,
    pub updated_by: i64,
    pub updated_at: String,
}

pub async fn add_and_update_entry(
    pool: &SqlitePool,
    leaderboard_id: i64,
    player_name: &str,
    player_link: Option<&str>,
    score: i64,
    stat_text: &str,
    stat_link: Option<&str>,
    note: &str,
    created_by: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO entries (leaderboard_id, player_name, player_link, score, stat_text, stat_link, note, created_by, updated_by, updated_at) \
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now')) ON CONFLICT (leaderboard_id, player_name) DO UPDATE SET \
        player_name = excluded.player_name, player_link = excluded.player_link, score = excluded.score, \
        stat_text = excluded.stat_text, stat_link = excluded.stat_link, note = excluded.note, updated_by = excluded.updated_by, updated_at = excluded.updated_at",
    )
    .bind(leaderboard_id)
    .bind(player_name)
    .bind(player_link)
    .bind(score)
    .bind(stat_text)
    .bind(stat_link)
    .bind(note)
    .bind(created_by)
    .bind(created_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_entry_by_id(
    pool: &SqlitePool,
    id: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "DELETE FROM entries WHERE id = ?"
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_entries(
    pool: &SqlitePool,
    leaderboard_id: i64,
    lower_is_better: bool,
) -> anyhow::Result<Vec<Entry>> {
    let order = if lower_is_better { "ASC" } else { "DESC" };
    let sql = format!(
        "SELECT id, leaderboard_id, player_name, player_link, score, stat_text, stat_link, note, \
        created_by, created_at, updated_by, updated_at \
        FROM entries WHERE leaderboard_id = ? ORDER BY score {order} LIMIT 100"
    );
    let rows = sqlx::query_as::<_, Entry>(&sql)
        .bind(leaderboard_id)
        .fetch_all(pool)
        .await?;
    Ok(rows)
}

pub async fn get_entry_by_id(
    pool: &SqlitePool,
    entry_id: i64,
) -> anyhow::Result<Option<Entry>> {
    let sql = "SELECT id, leaderboard_id, player_name, player_link, score, stat_text, stat_link, note, \
    created_by, created_at, updated_by, updated_at FROM entries WHERE id = ?";
    let row = sqlx::query_as::<_, Entry>(&sql)
        .bind(entry_id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}
