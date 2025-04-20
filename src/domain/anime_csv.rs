use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;
use crate::domain::anime::{Anime, AnimeStatus};

#[derive(Debug, Deserialize)]
pub struct AnimeCsv {
    pub title: String,
    pub episodes_watched: u32,
    pub total_episodes: Option<u32>,
    pub status: AnimeStatus,
    pub added_at: DateTime<Utc>,
}

impl From<AnimeCsv> for Anime {
    fn from(value: AnimeCsv) -> Self {
        Anime {
            id: Uuid::new_v4(),
            title: value.title,
            episodes_watched: value.episodes_watched,
            total_episodes: value.total_episodes,
            status: value.status,
            added_at: value.added_at,
        }
    }
}
