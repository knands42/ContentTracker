use std::str::FromStr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::domain::anime::{Anime, AnimeStatus};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AnimeModel {
    pub id: String,
    pub title: String,
    pub episodes_watched: i32,
    pub total_episodes: Option<i32>,
    pub status: String,
    pub added_at: DateTime<Utc>,
}


impl From<&AnimeModel> for Anime {
    fn from(model: &AnimeModel) -> Self {
        Anime {
            id: Uuid::parse_str(&model.id).unwrap(),
            title: model.title.clone(),
            episodes_watched: model.episodes_watched as u32,
            total_episodes: model.total_episodes.map(|x| x as u32),
            status: AnimeStatus::from_str(&model.status).unwrap(),
            added_at: model.added_at,

        }
    }
}