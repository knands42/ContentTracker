use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn get_db_pool() -> SqlitePool {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}...", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created successfully"),
            Err(e) => panic!("error: {}", e),
        }
    } else {
        println!("Database {} already exists", DB_URL);
    }

    SqlitePool::connect(DB_URL)
        .await
        .expect("Failed to connect to database")
}

pub async fn run_migrations(db: &SqlitePool) {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&manifest_dir).join("migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(db)
        .await;

    match migration_results {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => panic!("error: {}", e),
    };
}
