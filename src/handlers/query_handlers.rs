use std::sync::Arc;

use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws::{self, WsResponseBuilder};
use erased_serde::Serialize as ErasedSerialize;
use log::debug;
use sqlx::SqlitePool;

use crate::models::error_models::VaderError;
use crate::models::query_models::{EventInfo, FtsQuery, IdQuery, TeamInfo, Vboard};
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

#[get("/event/info/all")]
pub async fn get_all_event(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let res: Result<Vec<EventInfo>, VaderError> = EventInfo::get_all_event_info(&db_pool).await;
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

#[get("/team/info/all")]
pub async fn get_all_team(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let res: Result<Vec<TeamInfo>, VaderError> = TeamInfo::get_all_team_info(&db_pool).await;
    match res {
        Ok(event) => HttpResponse::Ok().json(web::Json(event)),
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

#[get("/user/info/all")]
pub async fn get_all_user(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let res: Result<Vec<User>, VaderError> = User::get_all_user(&db_pool).await;
    match res {
        Ok(event) => HttpResponse::Ok().json(web::Json(event)),
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
    stream: web::Payload,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to Fetch details")
    } else {
        let vb = Vboard {};
        let wsb_res = WsResponseBuilder::new(vb, &req, stream).start_with_addr();
        match wsb_res {
            Ok(ws_res) => {
                let mut addrs = app_state.vb_addr.lock().await;
                addrs.push(ws_res.0);
                ws_res.1
            }
            Err(e) => {
                log::error!(
                    "[Error] : Unable to build websocket responder : {}",
                    e.to_string()
                );
                HttpResponse::BadRequest().body("Unable to connect")
            }
        }
    }
}
