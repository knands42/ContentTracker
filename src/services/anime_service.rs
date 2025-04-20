use sqlx::SqlitePool;
use std::fs::File;

use crate::domain::anime::{Anime, AnimeStatus};
use crate::domain::anime_csv::AnimeCsv;
use crate::models::anime_model::AnimeModel;
use chrono::Utc;
use csv::Reader;
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

    pub async fn import_from_csv(&self, file_path: &str) -> Result<(), sqlx::Error> {
        let file = File::open(file_path)?;
        let mut reader = Reader::from_reader(file);

        for result in reader.deserialize() {
            let csv_anime: AnimeCsv = result.unwrap();
            let anime: Anime = csv_anime.into();

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
        }

        Ok(())
    }

    pub async fn get_animes(&self) -> Result<Vec<Anime>, sqlx::Error> {
        let rows = sqlx::query_as::<_, AnimeModel>(
            r#"
            SELECT 
                id, 
                title, 
                episodes_watched, 
                total_episodes, 
                status, 
                added_at 
            FROM animes
            ORDER BY added_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let animes = rows.iter().map(|x| x.into()).collect();

        Ok(animes)
    }
}
