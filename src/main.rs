use actix_web::web::Data;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
use std::sync::Arc;

mod handlers;
mod models;
mod services;

use crate::handlers::command_handlers::{
    add_event, add_team, add_team_members, add_user, end_event, start_event, update_score,
};
use crate::models::v_models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let db_url = env::var("DATABASE_URL").expect("Error Reading DATABASE_URL Env Variable");
    let host_port = format!("{}:{}", host, port);
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Error connecting to Database");
    log::info!("Database connection successful");
    log::info!("Server running at {}", host_port);
    let app_state = Arc::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(db_pool.clone()))
            .service(add_event)
            .service(add_user)
            .service(add_team)
            .service(add_team_members)
            .service(start_event)
            .service(update_score)
            .service(end_event)
    })
    .bind(host_port)?
    .run()
    .await
}
