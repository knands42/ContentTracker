use clap::{Parser, Subcommand};
use crate::services::AnimeService;
use crate::storage::{get_db_pool, run_migrations};

#[derive(Parser)]
#[command(name = "AniTracker")]
#[command(about = "Track your anime watchlist from the terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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


pub async fn initialize_cli() {
    let cli = Cli::parse();

    let pool = get_db_pool().await;
    run_migrations(&pool).await;

    let anime_service = AnimeService::new(pool.clone());

    match &cli.command {
        Commands::Add { title, episodes } => {
            match anime_service
                .add_anime(title.clone(), Some(*episodes))
                .await
            {
                Ok(anime) => println!("✅ Added: {}", anime.title),
                Err(e) => eprintln!("❌ Error adding anime: {}", e),
            }
        }
        Commands::List => {
            println!("Your anime list:");
        }
    }
}