mod cli;
mod models;
mod services;
mod storage;

use crate::cli::anime_cli::initialize_cli;

#[tokio::main]
async fn main() {
    initialize_cli().await;
}
