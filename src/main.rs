use chrono::Utc;
use clap::{Parser, Subcommand};
use uuid::Uuid;

mod models;
mod storage;
mod services;

use services::AnimeService;
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
    
    let anime_service = AnimeService::new(pool.clone());

    match &cli.command {
        Commands::Add { title, episodes } => {
            match anime_service.add_anime(title.clone(), Some(*episodes)).await {
                Ok(anime) => println!("✅ Added: {}", anime.title),
                Err(e) => eprintln!("❌ Error adding anime: {}", e),
            }
        }
        Commands::List => {
            println!("Your anime list:");
        }
    }
}
