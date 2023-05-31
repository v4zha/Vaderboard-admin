use std::sync::Arc;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use erased_serde::Serialize as ErasedSerialize;
use log::debug;
use sqlx::SqlitePool;

use crate::models::error_models::VaderError;
use crate::models::query_models::{EventInfo, FtsQuery, IdQuery, TeamInfo};
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

#[get("/event/fts")]
pub async fn event_fts(
    req: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    stream: web::Payload,
) -> impl Responder {
    ws::start(
        FtsQuery::<EventInfo>::new(db_pool.into_inner()),
        &req,
        stream,
    )
}

#[get("/team/fts")]
pub async fn team_fts<'a>(
    req: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    stream: web::Payload,
) -> impl Responder {
    ws::start(
        FtsQuery::<TeamInfo>::new(db_pool.into_inner()),
        &req,
        stream,
    )
}
#[get("/user/fts")]
pub async fn user_fts(
    req: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    stream: web::Payload,
) -> impl Responder {
    ws::start(FtsQuery::<User>::new(db_pool.into_inner()), &req, stream)
}

#[get("/vaderboard")]
pub async fn vaderboard(
    req: HttpRequest,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
    stream: web::Payload,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to Fetch details")
    } else {
        // ws::start(,&req,stream)
        todo!()
    }
}
