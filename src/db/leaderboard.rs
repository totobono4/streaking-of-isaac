use std::vec;

use serde::Serialize;
use sqlx::SqlitePool;

pub enum Character {
    Isaac,
    Magdalene,
    Cain,
    Judas,
    BlueBaby,
    Eve,
    Samson,
    Azazel,
    Lazarus,
    Eden,
    Lost,
    Lilith,
    Keeper,
    Apollyon,
    Forgotten,
    Bethany,
    Jacob,

    TaintedIsaac,
    TaintedMagdalene,
    TaintedCain,
    TaintedJudas,
    TaintedBlueBaby,
    TaintedEve,
    TaintedSamson,
    TaintedAzazel,
    TaintedLazarus,
    TaintedEden,
    TaintedLost,
    TaintedLilith,
    TaintedKeeper,
    TaintedApollyon,
    TaintedForgotten,
    TaintedBethany,
    TaintedJacob,

    RandomV1,
    RandomV2,
    FullRandom,

    Challenge,
    Save,
}

impl Character {
    pub fn get_string(&self) -> &'static str {
        match self {
            Self::Isaac => { "Isaac" },
            Self::Magdalene => { "Magdalene" },
            Self::Cain => { "Cain" },
            Self::Judas => { "Judas" },
            Self::BlueBaby => { "BlueBaby" },
            Self::Eve => { "Eve" },
            Self::Samson => { "Samson" },
            Self::Azazel => { "Azazel" },
            Self::Lazarus => { "Lazarus" },
            Self::Eden => { "Eden" },
            Self::Lost => { "Lost" },
            Self::Lilith => { "Lilith" },
            Self::Keeper => { "Keeper" },
            Self::Apollyon => { "Apollyon" },
            Self::Forgotten => { "Forgotten" },
            Self::Bethany => { "Bethany" },
            Self::Jacob => { "Jacob & Esau" },

            Self::TaintedIsaac => { "Tainted Isaac" },
            Self::TaintedMagdalene => { "Tainted Magdalene" },
            Self::TaintedCain => { "Tainted Cain" },
            Self::TaintedJudas => { "Tainted Judas" },
            Self::TaintedBlueBaby => { "TaintedBlueBaby" },
            Self::TaintedEve => { "Tainted Eve" },
            Self::TaintedSamson => { "Tainted Samson" },
            Self::TaintedAzazel => { "Tainted Azazel" },
            Self::TaintedLazarus => { "Tainted Lazarus" },
            Self::TaintedEden => { "Tainted Eden" },
            Self::TaintedLost => { "Tainted Lost" },
            Self::TaintedLilith => { "Tainted Lilith" },
            Self::TaintedKeeper => { "Tainted Keeper" },
            Self::TaintedApollyon => { "Tainted Apollyon" },
            Self::TaintedForgotten => { "Tainted Forgotten" },
            Self::TaintedBethany => { "Tainted Bethany" },
            Self::TaintedJacob => { "Tainted Jacob" },

            Self::RandomV1 => { "Random V1" },
            Self::RandomV2 => { "Random V2" },
            Self::FullRandom => { "Full Random" },

            Self::Challenge => { "Challenge" },
            Self::Save => { "Save" },
        }
    }

    pub fn get_vec() -> Vec<Self> {
        vec![
            Self::Isaac,
            Self::Magdalene,
            Self::Cain,
            Self::Judas,
            Self::BlueBaby,
            Self::Eve,
            Self::Samson,
            Self::Azazel,
            Self::Lazarus,
            Self::Eden,
            Self::Lost,
            Self::Lilith,
            Self::Keeper,
            Self::Apollyon,
            Self::Forgotten,
            Self::Bethany,
            Self::Jacob,

            Self::TaintedIsaac,
            Self::TaintedMagdalene,
            Self::TaintedCain,
            Self::TaintedJudas,
            Self::TaintedBlueBaby,
            Self::TaintedEve,
            Self::TaintedSamson,
            Self::TaintedAzazel,
            Self::TaintedLazarus,
            Self::TaintedEden,
            Self::TaintedLost,
            Self::TaintedLilith,
            Self::TaintedKeeper,
            Self::TaintedApollyon,
            Self::TaintedForgotten,
            Self::TaintedBethany,
            Self::TaintedJacob,

            Self::RandomV1,
            Self::RandomV2,
            Self::FullRandom,

            Self::Challenge,
            Self::Save,
        ]
    }
}

pub enum Goal {
    BlueBaby,
    TheLamb,
    Mother,
    TheBeast,

    BossRush,
    Hush,

    MegaSatan,
    Delirium,
    Greedier,

    FourGoals,
    SixGoals,
    RandomBoss,
    Hoarder,
    
    UltraHard,
    Speed,
    BloodyMary,
    DeleteThis,
    Backasswards,

    DeadGod,
}

impl Goal {
    pub fn get_string(&self) -> &'static str {
        match self {
            Self::BlueBaby => { "Blue Baby" },
            Self::TheLamb => { "The Lamb" },
            Self::Mother => { "Mother" },
            Self::TheBeast => { "The Beast" },

            Self::BossRush => { "Boss Rush" },
            Self::Hush => { "Hush" },

            Self::MegaSatan => { "Mega Satan" },
            Self::Delirium => { "Delirium" },
            Self::Greedier => { "Greedier" },

            Self::FourGoals => { "4 Goals" },
            Self::SixGoals => { "6 Goals" },
            Self::RandomBoss => { "Random Boss" },
            Self::Hoarder => { "Hoarder" },

            Self::UltraHard => { "Ultra Hard" },
            Self::Speed => { "SPEED!" },
            Self::BloodyMary => { "Bloody Mary" },
            Self::DeleteThis => { "DELETE THIS" },
            Self::Backasswards => { "Backasswards"},

            Self::DeadGod => { "Dead God" },
        }
    }

    pub fn get_vec() -> Vec<Self> {
        vec![
            Self::BlueBaby,
            Self::TheLamb,
            Self::Mother,
            Self::TheBeast,
            
            Self::BossRush,
            Self::Hush,

            Self::MegaSatan,
            Self::Delirium,
            Self::Greedier,

            Self::FourGoals,
            Self::SixGoals,
            Self::RandomBoss,
            Self::Hoarder,

            Self::UltraHard,
            Self::Speed,
            Self::BloodyMary,
            Self::DeleteThis,
            Self::Backasswards,

            Self::DeadGod,
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
