use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;

mod handlers;
mod models;
mod services;

use crate::handlers::event_handlers::add_event;
use crate::models::v_models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let host = env::var("HOST").expect("Error Reading HOST Env Variable");
    let port = env::var("PORT").expect("Error Reading PORT Env Variable");
    let db_url = env::var("DATABASE_URL").expect("Error Reading DATABASE_URL Env Variable");
    let host_port = format!("{}:{}", host, port);
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Error connecting to Database");
    log::info!("Database connection successful");
    log::info!("Server running at {}", host_port);
    let app_state = Data::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(db_pool.clone()))
            .service(add_event)
    })
    .bind(host_port)?
    .run()
    .await
}
