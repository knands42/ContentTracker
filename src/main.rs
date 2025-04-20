use chrono::Utc;
use clap::{Parser, Subcommand};
use uuid::Uuid;

mod models;
mod storage;
use crate::models::AnimeStatus;
use models::Anime;
use storage::{get_db_pool, run_migrations};

#[derive(Parser)]
#[command(name = "AniTracker")]
#[command(about = "Track your anime watchlist from the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        #[arg(short, long, default_value_t = 0)]
        episodes: u32,
    },
    List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let pool = get_db_pool().await;
    run_migrations(&pool).await;

    match &cli.command {
        Commands::Add { title, episodes } => {
            let anime = Anime {
                id: Uuid::new_v4(),
                title: title.clone(),
                episodes_watched: *episodes,
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
            .execute(&pool)
            .await
            .unwrap();

            println!("✅ Added: {}", anime.title);
        }
        Commands::List => {
            println!("Your anime list:");
        }
    }
}
