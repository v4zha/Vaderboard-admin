use crate::models::{
    handler_models::EventInfo,
    v_models::{Event, Player, Team, User, VaderEvent},
};
use actix_web::{web, HttpResponse, Responder};
use log::error;
use sqlx::SqlitePool;

async fn add_team_event(
    event_data: web::Json<EventInfo>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_info: EventInfo = event_data.into_inner();
    let event = Into::<Event<Team>>::into(event_info);
    match event.add_event(&db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            error!("Error adding Team event : {}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
async fn add_user_event(
    event_data: web::Json<EventInfo>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder {
    let event_info: EventInfo = event_data.into_inner();
    let event = Into::<Event<User>>::into(event_info);
    match event.add_event(&db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            error!("Error adding User event : {}", err.to_string());
            HttpResponse::InternalServerError().finish()
        }
    }
}
