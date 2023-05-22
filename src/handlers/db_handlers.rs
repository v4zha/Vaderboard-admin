use crate::models::{
    handler_models::EventInfo,
    v_models::{Event, Team, User, VaderEvent},
};
use actix_web::{post, web, HttpResponse, Responder};
use log::{error, info};
use sqlx::SqlitePool;

#[post("/event/team")]
pub async fn add_team_event(
    event_data: web::Json<EventInfo>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_info: EventInfo = event_data.into_inner();
    let event = Into::<Event<Team>>::into(event_info);
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added team Event [id : {}]", event.id);
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            error!("Error adding Team event : {}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
#[post("/event/user")]
pub async fn add_user_event(
    event_data: web::Json<EventInfo>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_info: EventInfo = event_data.into_inner();
    let event = Into::<Event<User>>::into(event_info);
    match event.add_event(&db_pool).await {
        Ok(_) => {
            info!("Successfully added user Event [id : {}]", event.id);
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            error!("Error adding User event : {}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
