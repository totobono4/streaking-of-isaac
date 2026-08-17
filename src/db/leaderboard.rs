use std::vec;

use serde::Serialize;
use sqlx::SqlitePool;

pub enum Character {
    Eden,
    Lost,
    TaintedKeeper,
    TaintedLost,
    RandomV1,
    RandomV2,

    None,
}

impl Character {
    pub fn get_string(&self) -> &'static str {
        match self {
            Self::Eden => { "Eden" },
            Self::Lost => { "Lost" },
            Self::TaintedKeeper => { "Tainted Keeper" },
            Self::TaintedLost => { "Tainted Lost" },
            Self::RandomV1 => { "Random V1" },
            Self::RandomV2 => { "Random V2" },

            Self::None => { "" },
        }
    }

    pub fn get_vec() -> Vec<Self> {
        vec![
            Self::Eden,
            Self::Lost,
            Self::TaintedKeeper,
            Self::TaintedLost,
            Self::RandomV1,
            Self::RandomV2,

            Self::None,
        ]
    }
}

pub enum Goal {
    Chest,
    Mother,
    FourGoals,
    RandomBoss,
    DeadGod,
    UltraHard,
    Speed,
    BloodyMary,
    DeleteThis,
    Backasswards,
}

impl Goal {
    pub fn get_string(&self) -> &'static str {
        match self {
            Self::Chest => { "Chest" },
            Self::Mother => { "Mother" },
            Self::FourGoals => { "4 Goals" },
            Self::RandomBoss => { "Random Boss" },
            Self::DeadGod => { "Dead God" },
            Self::UltraHard => { "Ultra Hard" },
            Self::Speed => { "SPEED!" },
            Self::BloodyMary => { "Bloody Mary" },
            Self::DeleteThis => { "DELETE THIS" },
            Self::Backasswards => { "Backasswards"},
        }
    }

    pub fn get_vec() -> Vec<Self> {
        vec![
            Self::Chest,
            Self::Mother,
            Self::FourGoals,
            Self::RandomBoss,
            Self::DeadGod,
            Self::UltraHard,
            Self::Speed,
            Self::BloodyMary,
            Self::DeleteThis,
            Self::Backasswards,
        ]
    }
}

pub enum GameVersion {
    RepentancePlus,
    Repentance,
    AfterbirthPlus,
}

impl GameVersion {
    pub fn get_string(&self) -> &'static str {
        match self {
            Self::RepentancePlus => { "Repentance+" },
            Self::Repentance => { "Repentance" },
            Self::AfterbirthPlus => { "Afterbirth+" },
        }
    }

    pub fn get_vec() -> Vec<Self> {
        vec![
            Self::RepentancePlus,
            Self::Repentance,
            Self::AfterbirthPlus,
        ]
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Leaderboard {
    pub id: i64,
    pub slug: String,
    pub character: String,
    pub goal: String,
    pub game_version: String,
    pub modifier: String,
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
    character: &str,
    goal: &str,
    game_version: &str,
    modifier: &str,
    description: Option<&str>,
    unit: &str,
    stat: &str,
    lower_is_better: bool,
    created_by: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO leaderboards (slug, character, goal, game_version, modifier, description, unit, stat, lower_is_better, created_by, updated_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(slug)
    .bind(character)
    .bind(goal)
    .bind(game_version)
    .bind(modifier)
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
    character: &str,
    goal: &str,
    game_version: &str,
    modifier: &str,
    description: Option<&str>,
    unit: &str,
    stat: &str,
    lower_is_better: bool,
    created_by: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE leaderboards SET slug = ?, character = ?, goal = ?, game_version = ?, modifier = ?, description = ?, unit = ?, stat = ?, lower_is_better = ?, updated_by = ?, updated_at = datetime('now') WHERE id = ?",
    )
    .bind(slug)
    .bind(character)
    .bind(goal)
    .bind(game_version)
    .bind(modifier)
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
        "SELECT id, slug, character, goal, game_version, modifier, description, unit, stat, lower_is_better, created_by, created_at, updated_by, updated_at FROM leaderboards ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_leaderboard_version_count(
    pool: &SqlitePool,
    game_version: &str,
) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar(
        "SELECT COUNT(*) FROM leaderboards WHERE version = ?",
    )
    .bind(game_version)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_leaderboard_version_character_count(
    pool: &SqlitePool,
    game_version: &str,
    character: &str,
) -> anyhow::Result<i64> {
    let count = sqlx::query_scalar(
        "SELECT COUNT(*) FROM leaderboards WHERE version = ?, character = ?",
    )
    .bind(game_version)
    .bind(character)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn get_leaderboard_by_slug(
    pool: &SqlitePool,
    slug: &str,
) -> anyhow::Result<Option<Leaderboard>> {
    let row = sqlx::query_as::<_, Leaderboard>(
        "SELECT id, slug, character, goal, game_version, modifier, description, unit, stat, lower_is_better, created_by, created_at, updated_by, updated_at FROM leaderboards WHERE slug = ?",
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
