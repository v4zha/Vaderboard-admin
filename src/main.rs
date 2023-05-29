use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::web::{self, Data};
use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;

use sqlx::SqlitePool;
use std::env;
use std::sync::Arc;

mod handlers;
mod models;
mod services;

use crate::handlers::command_handlers::{
    add_event, add_team, add_team_members, add_user, delete_event, delete_team, delete_user,
    end_event, login, reset_score, start_event, update_score,
};
use crate::handlers::query_handlers::{
    get_current_event, get_event_info, get_team_info, get_user_info,
};
use crate::models::v_models::AppState;
use crate::services::v_middlewares::AdminOnlyGuard;
use actix_files::Files;

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
    let session_key = Key::generate();
    let host_port = format!("{}:{}", host, port);
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Error connecting to Database");
    log::info!("Database connection successful");
    log::info!("Server running at {}", host_port);
    let app_state = Arc::new(AppState::new());
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/", "dist").index_file("index.html"))
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_key.clone(),
            ))
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(db_pool.clone()))
            .service(login)
            .service(
                web::scope("/admin")
                    .wrap(AdminOnlyGuard)
                    .service(add_event)
                    .service(add_user)
                    .service(add_team)
                    .service(add_team_members)
                    .service(start_event)
                    .service(update_score)
                    .service(reset_score)
                    .service(get_user_info)
                    .service(delete_event)
                    .service(delete_team)
                    .service(delete_user)
                    .service(end_event),
            )
            .service(get_current_event)
            .service(get_event_info)
            .service(get_team_info)
    })
    .bind(host_port)?
    .run()
    .await
}
