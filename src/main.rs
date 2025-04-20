mod cli;
mod domain;
mod services;
mod storage;
mod models;

use crate::cli::anime_cli::initialize_cli;

#[tokio::main]
async fn main() {
    initialize_cli().await;
}
