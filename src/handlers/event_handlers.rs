use crate::models::{
    handler_models::{ContestantInfo, CreationResponse, EventInfo, MemberInfo, ScoreUpdate},
    v_models::{AppState, Event, EventStateWrapper, EventWrapper, Team, User, VaderEvent},
};
use actix_web::{post, web, Either, HttpResponse, Responder};
use log::{debug, error, info};
use sqlx::SqlitePool;

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
    let event_id = event.id;
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added team Event [id : {}]", event_id);
            *event_state = Some(EventWrapper::TeamEvent(EventStateWrapper::New(event)));
            HttpResponse::Ok().json(web::Json(CreationResponse::new(
                "Successfully added team event",
                event_id,
            )))
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
    let event_id = event.id;
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added user Event [id : {}]", event_id);
            *event_state = Some(EventWrapper::UserEvent(EventStateWrapper::New(event)));
            HttpResponse::Ok().json(web::Json(CreationResponse::new(
                "Successfully added user Event",
                event_id,
            )))
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

#[post("/score/update")]
pub async fn update_score(
    score_req: web::Json<ScoreUpdate>,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let sr = score_req.into_inner();
        let res = event_state
            .as_ref()
            .unwrap()
            .update_score_by_id(&sr.id, sr.score, &db_pool)
            .await;
        match res {
            Ok(_) => {
                info!("Score updated successfully");
                HttpResponse::Ok().body("Score updated successfully")
            }
            Err(err) => {
                debug!("Error updating score :\n[error] : {}", err);
                HttpResponse::BadRequest().body("Error updating Score")
            }
        }
    }
}
// ippol participants um contestants um onnaaan : )
#[post("/event/team/add")]
pub async fn add_team(
    c_info: web::Json<ContestantInfo>,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let team = Into::<Team>::into(c_info.into_inner());
        let team_id = team.id;
        let res = event_state.as_ref().unwrap().add_team(team, &db_pool).await;
        match res {
            Ok(_) => {
                info!("Team  added successfully");
                HttpResponse::Ok().json(web::Json(CreationResponse::new(
                    "Team added successfully",
                    team_id,
                )))
            }
            Err(err) => {
                debug!("Error adding Team :\n[error] : {}", err);
                HttpResponse::BadRequest().body("Error adding Team")
            }
        }
    }
}

// ippol participants um contestants um onnaaan : )
#[post("/event/user/add/")]
pub async fn add_user(
    c_info: web::Json<ContestantInfo>,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let user = Into::<User>::into(c_info.into_inner());
        let user_id = user.id;
        let res = event_state.as_ref().unwrap().add_user(user, &db_pool).await;
        match res {
            Ok(_) => {
                info!("User  added successfully");
                HttpResponse::Ok().json(web::Json(CreationResponse::new(
                    "User added successfully",
                    user_id,
                )))
            }
            Err(err) => {
                debug!("Error adding User :\n[error] : {}", err);
                HttpResponse::BadRequest().body("Error adding User")
            }
        }
    }
}

#[post("/event/team/add_members")]
pub async fn add_team_members(
    m_info: web::Json<MemberInfo>,
    app_state: web::Data<AppState>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state
        .current_event
        .lock()
        .expect("Error getting current Event lock");
    if event_state.is_none() {
        debug!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let mi = m_info.into_inner();
        let res = event_state
            .as_ref()
            .unwrap()
            .add_team_members(&mi, &db_pool)
            .await;
        match res {
            Ok(_) => {
                info!("Team  added successfully");
                HttpResponse::Ok().body("Team added successfully")
            }
            Err(err) => {
                debug!("Error adding Team :\n[error] : {}", err);
                HttpResponse::BadRequest().body("Error adding Team")
            }
        }
    }
}
