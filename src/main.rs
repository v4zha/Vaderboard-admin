use std::env;
use std::sync::Arc;

use actix::Actor;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::{self, Data};
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::SqlitePool;

mod handlers;
mod models;
mod services;

use actix_files::Files;

use crate::handlers::command_handlers::{
    add_event, add_team, add_team_members, add_team_with_members, add_user, delete_event,
    delete_team, delete_user, end_event, login, reset_score, start_event, update_score,
};
use crate::handlers::query_handlers::{
    event_fts, get_all_event, get_all_team, get_all_user, get_current_event, get_event_info,
    get_event_rem_members, get_event_teams, get_event_users, get_team_info, get_user_info,
    team_fts, user_fts, vaderboard,
};
use crate::models::query_models::{CurFtsServer, VboardSrv};
use crate::models::v_models::AppState;
use crate::services::v_middlewares::AdminOnlyGuard;

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
    let vb_count: u32 = env::var("VADERBOARD_COUNT").map_or(10, |count| {
        count
            .parse::<u32>()
            .expect("Unable to parse VADERBOARD_COUNT,please replace with a positive integer")
    });
    let session_key = Key::generate();
    let host_port = format!("{}:{}", host, port);
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Error connecting to Database");
    let app_state = Arc::new(AppState::new(vb_count));
    //VaderBoard server Actor
    let vb_srv = VboardSrv::new(app_state.clone(), db_pool.clone()).start();
    //Current Event Fts Actor
    let cur_fts = CurFtsServer::new().start();
    log::info!("Database connection successful");
    log::info!("Server Starting on :  {}", host_port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                session_key.clone(),
            ))
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(vb_srv.clone()))
            .app_data(Data::new(cur_fts.clone()))
            .app_data(Data::new(db_pool.clone()))
            .service(login)
            .service(
                web::scope("/admin")
                    .wrap(AdminOnlyGuard)
                    .service(add_event)
                    .service(add_user)
                    .service(add_team)
                    .service(add_team_members)
                    .service(add_team_with_members)
                    .service(start_event)
                    .service(update_score)
                    .service(reset_score)
                    .service(delete_event)
                    .service(delete_team)
                    .service(delete_user)
                    .service(end_event),
            )
            .service(get_current_event)
            .service(get_event_teams)
            .service(get_event_rem_members)
            .service(get_event_users)
            .service(get_all_event)
            .service(get_event_info)
            .service(get_all_team)
            .service(get_team_info)
            .service(get_all_user)
            .service(get_user_info)
            .service(event_fts)
            .service(team_fts)
            .service(user_fts)
            .service(vaderboard)
            .service(Files::new("/", "dist").index_file("index.html"))
    })
    .bind(host_port)?
    .run()
    .await
}
