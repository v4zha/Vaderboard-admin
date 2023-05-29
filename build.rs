use dotenvy::dotenv;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use std::env;
use std::process::Command;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Running Vaderboard Build script : )");
    //VaderBoard database setup
    println!("Running Database Setup");
    let db_url = env::var("DATABASE_URL")?;
    if !Sqlite::database_exists(&db_url).await? {
        println!("Creating Vaderboard database");
        Sqlite::create_database(&db_url).await?;
    }
    let pool = SqlitePool::connect(&db_url).await?;
    println!("Running Vaderboard Migrations");
    sqlx::migrate!().run(&pool).await?;

    //Vite build setup
    println!("Running vader-admin-ui Build");
    let vite_res = Command::new("npm")
        .arg("--prefix")
        .arg("vader-admin-ui")
        .arg("run")
        .arg("build")
        .output()
        .expect("Failed building vader-admin-ui");
    if vite_res.status.success() {
        println!("Build Successful");
    } else {
        println!(
            "Build Error : {}",
            String::from_utf8_lossy(&vite_res.stderr)
        );
    }
    println!("Build script Executed Successfully : )");
    Ok(())
}
