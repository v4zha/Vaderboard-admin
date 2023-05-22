use crate::models::{
    handler_models::EventInfo,
    v_models::{AppState, Event, EventStateWrapper, EventWrapper, Team, User, VaderEvent},
};
use actix_web::{post, web, Either, HttpResponse, Responder};
use log::{debug, error, info};
use sqlx::SqlitePool;
use uuid::Uuid;

#[post("/event/add")]
pub async fn add_event(
    event_data: web::Json<EventInfo>,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> Either<impl Responder, impl Responder> {
    let event_data = event_data.into_inner();
    match event_data.event_type {
        crate::models::handler_models::EventType::TeamEvent => {
            Either::Left(add_team_event(event_data, app_state, db_pool).await)
        }
        crate::models::handler_models::EventType::UserEvent => {
            Either::Right(add_user_event(event_data, app_state, db_pool).await)
        }
    }
}

pub async fn add_team_event(
    event_info: EventInfo,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let mut event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_some() {
        debug!("Request delined.Another Event already added.");
        return HttpResponse::BadRequest()
            .body("Another event already Added . Wait till the current Event ends");
    }
    let event = Into::<Event<Team>>::into(event_info);
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added team Event [id : {}]", event.id);
            *event_state = Some(EventWrapper::TeamEvent(EventStateWrapper::New(event)));
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            error!("Error adding Team event : {}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
pub async fn add_user_event(
    event_info: EventInfo,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let mut event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_some() {
        debug!("Request delined.Another Event already added.");
        return HttpResponse::BadRequest()
            .body("Another event already Added . Wait till the current Event ends");
    }
    let event = Into::<Event<User>>::into(event_info);
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added user Event [id : {}]", event.id);
            *event_state = Some(EventWrapper::UserEvent(EventStateWrapper::New(event)));
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            error!("Error adding User event : {}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/event/start")]
pub async fn start_event(app_state: web::Data<AppState>) -> impl Responder {
    let mut event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        event_state.as_mut().unwrap().start_event();
        let body = format!(
            "Event id : [{}] started successfully",
            event_state.as_ref().unwrap().get_id()
        );
        info!("{}", body);
        HttpResponse::Ok().body(body)
    }
}
#[post("/event/stop")]
pub async fn end_event(app_state: web::Data<AppState>) -> impl Responder {
    let mut event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        event_state.as_mut().unwrap().end_event();
        let body = format!(
            "Event id : [{}] stopped successfully",
            event_state.as_ref().unwrap().get_id()
        );
        info!("{}", body);
        HttpResponse::Ok().body(body)
    }
}




