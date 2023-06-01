use std::sync::Arc;

use actix_session::Session;
use actix_web::{post, web, Either, HttpResponse, Responder};
use log::{error, info};
use sqlx::SqlitePool;

use crate::models::command_models::{
    CommandResponse, ContestantInfo, EventReq, MemberInfo, ScoreUpdate,
};
use crate::models::error_models::VaderError;
use crate::models::query_models::{EventInfo, EventType, IdQuery, VboardRes};
use crate::models::v_models::{AdminInfo, AppState, Event, Team, User, VaderEvent};
use crate::models::wrapper_models::{EventStateWrapper, EventWrapper};

#[post("/event/add")]
pub async fn add_event<'a>(
    event_data: web::Json<EventReq<'a>>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> Either<impl Responder, impl Responder>
where
    'a: 'static,
{
    let event_data = event_data.into_inner();
    match event_data.event_type {
        EventType::TeamEvent => Either::Left(add_team_event(event_data, app_state, db_pool).await),
        EventType::UserEvent => Either::Right(add_user_event(event_data, app_state, db_pool).await),
    }
}

pub async fn add_team_event<'a>(
    event_info: EventReq<'a>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder
where
    'a: 'static,
{
    let mut event_state = app_state.current_event.lock().await;
    if event_state.is_some() {
        error!("Request delined.Another Event already added.");
        return HttpResponse::BadRequest()
            .body("Another event already Added . Wait till the current Event ends");
    }
    let event = Into::<Event<Team>>::into(event_info);
    let event_id = event.id;
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added team Event [id : {}]", event_id);
            *event_state = Some(EventWrapper::TeamEvent(EventStateWrapper::New(event)));
            HttpResponse::Ok().json(web::Json(CommandResponse::new(
                "Successfully added team event",
                event_id,
            )))
        }
        Err(err) => {
            error!("Error adding Team event : {}", err.to_string());
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
pub async fn add_user_event<'a>(
    event_info: EventReq<'a>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder
where
    'a: 'static,
{
    let mut event_state = app_state.current_event.lock().await;
    if event_state.is_some() {
        error!("Request delined.Another Event already added.");
        return HttpResponse::BadRequest()
            .body("Another event already Added . Wait till the current Event ends");
    }
    let event = Into::<Event<User>>::into(event_info);
    let event_id = event.id;
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added user Event [id : {}]", event_id);
            *event_state = Some(EventWrapper::UserEvent(EventStateWrapper::New(event)));
            HttpResponse::Ok().json(web::Json(CommandResponse::new(
                "Successfully added user Event",
                event_id,
            )))
        }
        Err(err) => {
            error!("Error adding User event : {}", err.to_string());
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[post("/event/start")]
pub async fn start_event(
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let mut event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        //reset score before starting event
        let reset_res = event_state.as_ref().unwrap().reset_score(&db_pool).await;
        if let Err(e) = reset_res {
            error!("Error reseting score to start event");
            return HttpResponse::BadRequest().body(format!(
                "Error resetting score to starte event.\n{}",
                e.to_string()
            ));
        }
        let res = event_state.as_mut().unwrap().start_event();
        match res {
            Ok(_) => {
                let body = format!(
                    "Event id : [{}] started successfully",
                    event_state.as_ref().unwrap().get_id()
                );
                info!("{}", body);
                HttpResponse::Ok().body(body)
            }
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}
#[post("/event/stop")]
pub async fn end_event(app_state: web::Data<Arc<AppState>>) -> impl Responder {
    let mut event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let res = event_state.as_mut().unwrap().end_event();
        match res {
            Ok(_) => {
                let body = format!(
                    "Event id : [{}] stopped successfully",
                    event_state.as_ref().unwrap().get_id()
                );
                info!("{}", body);
                *event_state = None;
                HttpResponse::Ok().body(body)
            }
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}

#[post("/score/update")]
pub async fn update_score(
    score_req: web::Json<ScoreUpdate>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let sr = score_req.into_inner();
        let score_res = event_state
            .as_ref()
            .unwrap()
            .update_score_by_id(&sr.id, sr.score, &db_pool)
            .await;
        match score_res {
            Ok(_) => {
                info!("Score updated successfully");
                let vb_res = event_state.as_ref().unwrap().get_vboard(&db_pool, 10).await;
                match vb_res {
                    Ok(vb_str) => {
                        let vb_addr = app_state.vb_addr.lock().await;
                        vb_addr
                            .iter()
                            .for_each(|addr| addr.do_send(VboardRes(vb_str.clone())));
                    }
                    Err(e) => log::debug!("Error sending Vaderboard : {}", e),
                }
                HttpResponse::Ok().body("Score updated successfully")
            }
            Err(err) => {
                error!("Error updating score :\n[error] : {}", err);
                HttpResponse::BadRequest().body(format!("Error updating Score : \n{}", err))
            }
        }
    }
}
#[post("/score/reset")]
pub async fn reset_score(
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let res = event_state.as_ref().unwrap().reset_score(&db_pool).await;
        match res {
            Ok(_) => {
                info!("Score Reset successful");
                HttpResponse::Ok().body("Score reset successful")
            }
            Err(err) => {
                error!("Error resetting score :\n[error] : {}", err);
                HttpResponse::BadRequest().body(format!("Error resetting Score : \n{}", err))
            }
        }
    }
}
#[post("/event/team/add")]
pub async fn add_team(
    c_info: web::Json<ContestantInfo<'_>>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let team = Into::<Team>::into(c_info.into_inner());
        let team_id = team.id;
        let res = event_state.as_ref().unwrap().add_team(team, &db_pool).await;
        match res {
            Ok(_) => {
                info!("Team  added successfully : {}", team_id);
                HttpResponse::Ok().json(web::Json(CommandResponse::new(
                    "Team added successfully",
                    team_id,
                )))
            }
            Err(err) => {
                error!("Error adding Team :\n[error] : {}", err);
                HttpResponse::BadRequest().body(err.to_string())
            }
        }
    }
}

#[post("/event/user/add/")]
pub async fn add_user(
    c_info: web::Json<ContestantInfo<'_>>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
        HttpResponse::BadRequest().body("No event added.Add event to start event")
    } else {
        let user = Into::<User>::into(c_info.into_inner());
        let user_id = user.id;
        let res = event_state
            .as_ref()
            .unwrap()
            .add_user(&user, &db_pool)
            .await;
        match res {
            Ok(_) => {
                info!("User  added successfully : {}", user_id);
                HttpResponse::Ok().json(web::Json(CommandResponse::new(
                    "User added successfully",
                    user_id,
                )))
            }
            Err(err) => {
                error!("Error adding User :\n[error] : {}", err);
                HttpResponse::BadRequest().body("Error adding User")
            }
        }
    }
}

#[post("/event/team/add_members")]
pub async fn add_team_members(
    m_info: web::Json<MemberInfo>,
    app_state: web::Data<Arc<AppState>>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_state = app_state.current_event.lock().await;
    if event_state.is_none() {
        error!("Request delined.No event added");
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
                info!("Team Members added successfully : {:?}", mi.members);
                HttpResponse::Ok().body("Team Members added successfully")
            }
            Err(err) => {
                error!("Error adding Team Members:\n[error] : {}", err);
                HttpResponse::BadRequest().body(format!("Error adding Team Members : {}", err))
            }
        }
    }
}

#[post("/event/delete")]
pub async fn delete_event(
    db_pool: web::Data<SqlitePool>,
    app_state: web::Data<Arc<AppState>>,
    id_info: web::Json<IdQuery>,
) -> impl Responder {
    let id = id_info.into_inner().id;
    let event_state = app_state.current_event.lock().await;
    if let Some(e) = event_state.as_ref() {
        let event_id = e.get_id();
        if event_id.eq(&id) {
            return HttpResponse::BadRequest().body(
                VaderError::EventActive(
                    "Unable to remove Event i.e currently Added/Active.Stop the event to remove",
                )
                .to_string(),
            );
        }
    }
    let info_res: Result<EventInfo, VaderError> = EventInfo::get_event_info(&id, &db_pool).await;
    let res = match info_res {
        Ok(event) => match event.event_type {
            EventType::TeamEvent => Event::<Team>::delete_event(&id, &db_pool).await,
            EventType::UserEvent => Event::<User>::delete_event(&id, &db_pool).await,
        },
        Err(e) => {
            let err = format!("unable to get event info\n.{}", e);
            error!("{}", err);
            return HttpResponse::BadRequest().body(err);
        }
    };
    match res {
        Ok(_) => {
            info!("Successfully deleted event : {}", id);
            HttpResponse::Ok().json(web::Json(CommandResponse::new(
                "Successfully deleted event",
                id,
            )))
        }
        Err(e) => {
            let err = format!("Error Deleting event : {}.\n{}", id, e.to_string());
            error!("{}", err);
            HttpResponse::BadRequest().body(err)
        }
    }
}

#[post("/team/delete")]
pub async fn delete_team(
    db_pool: web::Data<SqlitePool>,
    id_info: web::Json<IdQuery>,
) -> impl Responder {
    let id = id_info.into_inner().id;
    let res = Team::delete_team(&id, &db_pool).await;
    match res {
        Ok(_) => {
            info!("Successfully deleted team : {}", id);
            HttpResponse::Ok().json(web::Json(CommandResponse::new(
                "Successfully deleted team ",
                id,
            )))
        }

        Err(e) => {
            let err = format!("Error Deleting team : {}.\n{}", id, e.to_string());
            error!("{}", err);
            HttpResponse::BadRequest().body(err)
        }
    }
}

#[post("/user/delete")]
pub async fn delete_user(
    db_pool: web::Data<SqlitePool>,
    id_info: web::Json<IdQuery>,
) -> impl Responder {
    let id = id_info.into_inner().id;
    let res = User::delete_user(&id, &db_pool).await;
    match res {
        Ok(_) => {
            info!("Successfully deleted user : {}", id);
            HttpResponse::Ok().json(web::Json(CommandResponse::new(
                "Successfully deleted user",
                id,
            )))
        }
        Err(e) => {
            let err = format!("Error Deleting user : {}.\n{}", id, e.to_string());
            error!("{}", err);
            HttpResponse::BadRequest().body(err)
        }
    }
}

#[post("/login")]
pub async fn login(
    session: Session,
    login_info: web::Json<AdminInfo>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let login = login_info.into_inner();
    match login.verify_passwd(&db_pool).await {
        Ok(true) => {
            if session.insert("admin", true).is_ok() {
                log::debug!("Login Successful : )");
                HttpResponse::Ok().body("Login Successful")
            } else {
                log::debug!("Unable to get Admin Session");
                HttpResponse::InternalServerError().finish()
            }
        }
        Ok(false) => {
            log::debug!("Invalid UserName/Password");
            HttpResponse::Unauthorized().body("Invalid UserName/Password")
        }
        Err(e) => {
            log::debug!("Admin Auth error : {}", e.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
