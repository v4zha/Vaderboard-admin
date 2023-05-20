use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Running Vaderboard Build script : )");
    let db_url = env::var("DATABASE_URL")?;
    if !Sqlite::database_exists(&db_url).await? {
        println!("Creating Vaderboard database");
        Sqlite::create_database(&db_url).await?;
    }
    let pool = SqlitePool::connect(&db_url).await?;
    println!("Running Vaderboard Migrations");
    sqlx::migrate!().run(&pool).await?;
    println!("Build script Executed Successfully : )");
    Ok(())
}
