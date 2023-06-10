use std::sync::Arc;

use actix::Addr;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use erased_serde::Serialize as ErasedSerialize;
use log::debug;
use sqlx::SqlitePool;

use crate::models::error_models::VaderError;
use crate::models::query_models::{
    CurFtsBuilder, EventInfo, FtsQuery, IdQuery, TeamInfo, VboardClient, VboardSrv,
};
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
#[get("event/info/team/{count}")]
pub async fn get_event_teams(
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
    count: web::Path<u32>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        Ok(HttpResponse::BadRequest().body("No event added.Add event to Fetch details"))
    } else {
        let event = event_state.as_ref().unwrap();
        match event {
            crate::models::wrapper_models::EventWrapper::TeamEvent(_) => {
                let event_id = event.get_id();
                let cur_fts =
                    CurFtsBuilder::<Team>::new(event_id, count.into_inner(), db_pool.into_inner())
                        .team_fts()
                        .build();
                ws::start(cur_fts, &req, stream)
            }
            crate::models::wrapper_models::EventWrapper::UserEvent(_) => {
                Ok(HttpResponse::BadRequest().body(
                    VaderError::EventTypeMismatch("Cannot get Team Info in user event").to_string(),
                ))
            }
        }
    }
}
#[get("event/info/team/rem_members/{count}")]
pub async fn get_event_rem_members(
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
    count: web::Path<u32>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        Ok(HttpResponse::BadRequest().body("No event added.Add event to Fetch details"))
    } else {
        let event = event_state.as_ref().unwrap();
        match event {
            crate::models::wrapper_models::EventWrapper::TeamEvent(_) => {
                let event_id = event.get_id();
                let cur_fts =
                    CurFtsBuilder::<Team>::new(event_id, count.into_inner(), db_pool.into_inner())
                        .rem_user_fts()
                        .build();
                ws::start(cur_fts, &req, stream)
            }
            crate::models::wrapper_models::EventWrapper::UserEvent(_) => {
                Ok(HttpResponse::BadRequest().body(
                    VaderError::EventTypeMismatch("Cannot get Team Info in user event").to_string(),
                ))
            }
        }
    }
}

#[get("event/info/user/{count}")]
pub async fn get_event_users(
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
    count: web::Path<u32>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        Ok(HttpResponse::BadRequest().body("No event added.Add event to Fetch details"))
    } else {
        let event = event_state.as_ref().unwrap();
        match event {
            crate::models::wrapper_models::EventWrapper::TeamEvent(_) => {
                Ok(HttpResponse::BadRequest().body(
                    VaderError::EventTypeMismatch("Cannot get User Info in team event").to_string(),
                ))
            }
            crate::models::wrapper_models::EventWrapper::UserEvent(_) => {
                let event_id = event.get_id();
                let cur_fts =
                    CurFtsBuilder::<User>::new(event_id, count.into_inner(), db_pool.into_inner())
                        .build();
                ws::start(cur_fts, &req, stream)
            }
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

#[get("/event/fts/{count}")]
pub async fn event_fts(
    req: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    count: web::Path<u32>,
    stream: web::Payload,
) -> impl Responder {
    ws::start(
        FtsQuery::<EventInfo>::new(count.into_inner(), db_pool.into_inner()),
        &req,
        stream,
    )
}

#[get("/team/fts/{count}")]
pub async fn team_fts<'a>(
    req: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    count: web::Path<u32>,
    stream: web::Payload,
) -> impl Responder {
    ws::start(
        FtsQuery::<TeamInfo>::new(count.into_inner(), db_pool.into_inner()),
        &req,
        stream,
    )
}
#[get("/user/fts/{count}")]
pub async fn user_fts(
    req: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    count: web::Path<u32>,
    stream: web::Payload,
) -> impl Responder {
    ws::start(
        FtsQuery::<User>::new(count.into_inner(), db_pool.into_inner()),
        &req,
        stream,
    )
}

#[get("/vaderboard")]
pub async fn vaderboard(
    req: HttpRequest,
    app_state: web::Data<Arc<AppState>>,
    srv_addr: web::Data<Addr<VboardSrv>>,
    stream: web::Payload,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        debug!("Request delined.No event added");
        Ok(HttpResponse::BadRequest().body("No event added.Add event to Fetch details"))
    } else {
        ws::start(VboardClient::new(srv_addr), &req, stream)
    }
}
