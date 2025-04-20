use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Anime {
    pub id: Uuid,
    pub title: String,
    pub episodes_watched: u32,
    pub total_episodes: Option<u32>,
    pub status: AnimeStatus,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AnimeStatus {
    Watching,
    Completed,
    Paused,
    Dropped,
    PlanToWatch,
}

impl FromStr for AnimeStatus {
    type Err = String;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "watching" => Ok(AnimeStatus::Watching),
            "completed" => Ok(AnimeStatus::Completed),
            "paused" => Ok(AnimeStatus::Paused),
            "dropped" => Ok(AnimeStatus::Dropped),
            "plan to watch" => Ok(AnimeStatus::PlanToWatch),
            _ => Err(format!("Invalid status: {}", input)),       
        }
    }
}

impl fmt::Display for AnimeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnimeStatus::Watching => "Watching",
                AnimeStatus::Completed => "Completed",
                AnimeStatus::Paused => "Paused",
                AnimeStatus::Dropped => "Dropped",
                AnimeStatus::PlanToWatch => "Plan to Watch",
            }
        )
    }
}
