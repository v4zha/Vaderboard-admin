use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use std::env;

mod handlers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    let host = env::var("HOST").expect("Error Reading HOST Env Variable");
    let port = env::var("PORT").expect("Error Reading PORT Env Variable");
    let db_url = env::var("DATABASE_URL").expect("Error Reading DATABASE_URL Env Variable");
    let host_port = format!("{}:{}", host, port);
    let db_pool = connct_db(&db_url)
        .await
        .expect("Error connecting to Database");
    log::info!("Database connection successful");
    log::info!("Server running at {}", host_port);
    HttpServer::new(move || App::new().wrap(Logger::default()).app_data(db_pool.clone()))
        .bind(host_port)?
        .run()
        .await
}

async fn connct_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists(db_url).await? {
        Sqlite::create_database(db_url).await?;
    }
    let pool = SqlitePool::connect(db_url).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
