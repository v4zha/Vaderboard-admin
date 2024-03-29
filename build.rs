use std::env;
use std::process::Command;

use bcrypt::{hash, BcryptError, DEFAULT_COST};
use dotenvy::dotenv;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

async fn add_admin<'a>(
    uname: String,
    pass: String,
    db_pool: SqlitePool,
) -> Result<(), BcryptError> {
    // why no argon2 , veruthe : )
    let passwd = hash(pass, DEFAULT_COST)?;
    sqlx::query!(
        "INSERT OR IGNORE INTO admin_login (username,password) VALUES (?,?)",
        uname,
        passwd,
    )
    .execute(&db_pool)
    .await
    .expect("Error adding admin cred to admin_login");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Running Vaderboard Build script : )");
    //VaderBoard database setup
    println!("Running Database Setup");
    let db_url = env::var("DATABASE_URL").expect("Error DATABASE_URL env variable not set");
    if !Sqlite::database_exists(&db_url)
        .await
        .expect("Unable to fetch database existance details")
    {
        println!("Creating Vaderboard database");
        Sqlite::create_database(&db_url)
            .await
            .expect("Unable to create database");
    }
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Unable to Connect to Database");
    println!("Running Vaderboard Migrations");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run Db migrations");

    //add Admin Login to DB
    println!("Adding Admin cred to db");
    let uname = env::var("ADMIN_USERNAME").expect("Error ADMIN_USERNAME env variable not set");
    let pass = env::var("ADMIN_PASSWORD").expect("Error ADMIN_PASSWORD env variable not set");
    add_admin(uname, pass, pool).await?;
    println!("Successfully registered admin cred");

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
