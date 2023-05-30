use std::sync::Arc;

use actix_web::{get, web, HttpResponse, Responder};
use erased_serde::Serialize as ErasedSerialize;
use log::debug;
use sqlx::SqlitePool;

use crate::models::error_models::VaderError;
use crate::models::query_models::{EventInfo, IdQuery};
use crate::models::v_models::{AppState, Team, User};

#[get("/event/info")]
pub async fn get_current_event(
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to Fetch details")
    } else {
        let res: Result<Box<dyn ErasedSerialize>, VaderError> =
            event_state.as_ref().unwrap().get_event(&db_pool).await;
        match res {
            Ok(res) => HttpResponse::Ok().json(web::Json(res)),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}
#[get("/event/info/id")]
pub async fn get_event_info(
    id_info: web::Json<IdQuery>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let id = id_info.into_inner().id;
    let res: Result<EventInfo, VaderError> = EventInfo::get_event_info(&id, &db_pool).await;
    match res {
        Ok(event) => HttpResponse::Ok().json(web::Json(event)),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
#[get("/team/info")]
pub async fn get_team_info(
    id_info: web::Json<IdQuery>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let id = id_info.into_inner().id;
    match Team::get_team(&id, &db_pool).await {
        Ok(team) => HttpResponse::Ok().json(web::Json(team)),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("/user/info")]
pub async fn get_user_info(
    id_info: web::Json<IdQuery>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let id = id_info.into_inner().id;
    match User::get_user(&id, &db_pool).await {
        Ok(user) => HttpResponse::Ok().json(web::Json(user)),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("/events/fts")]
pub async fn events_fts(db_pool: web::Data<SqlitePool>, steam: web::Payload) -> impl Responder {}

#[get("/team/fts")]
pub async fn team_fts(db_pool: web::Data<SqlitePool>, steam: web::Payload) -> impl Responder {}

#[get("/users/fts")]
pub async fn users_fts(db_pool: web::Data<SqlitePool>, steam: web::Payload) -> impl Responder {}
