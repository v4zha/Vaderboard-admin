use crate::models::{
    handler_models::{DynEvent, EventInfo, EventType},
    v_models::Player,
};
use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

fn add_event<'a, T>(
    event_data: web::Json<EventInfo>,
    db_pool: web::Data<SqlitePool>,
) -> impl Responder
where
    T: Player<'a>,
{
    let event_info: EventInfo = event_data.into_inner();
    let event = Into::<DynEvent>::into(event_info);
    HttpResponse::Ok()
}
