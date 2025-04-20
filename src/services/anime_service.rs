use sqlx::SqlitePool;

use crate::models::{Anime, AnimeStatus};
use chrono::Utc;
use uuid::Uuid;

pub struct AnimeService {
    pool: SqlitePool,
}

impl AnimeService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn add_anime(
        &self,
        title: String,
        episodes: Option<u32>,
    ) -> Result<Anime, sqlx::Error> {
        let anime = Anime {
            id: Uuid::new_v4(),
            title: title.clone(),
            episodes_watched: episodes.unwrap_or(1),
            total_episodes: None,
            status: AnimeStatus::Watching,
            added_at: Utc::now(),
        };

        sqlx::query(
            r#"
                INSERT INTO animes (id, title, episodes_watched, total_episodes, status, added_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
        )
        .bind(anime.id.to_string())
        .bind(&anime.title)
        .bind(anime.episodes_watched as i32)
        .bind(anime.total_episodes.map(|x| x as i32))
        .bind(&anime.status.to_string())
        .bind(anime.added_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(anime)
    }
}
