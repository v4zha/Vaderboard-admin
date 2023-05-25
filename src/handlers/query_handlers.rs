use crate::models::{error_models::VaderError, v_models::AppState};
use actix_web::{get, web, HttpResponse, Responder};
use erased_serde::Serialize as ErasedSerialize;
use log::{debug, error};
use sqlx::SqlitePool;
use std::sync::Arc;

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
            Ok(res) => {
                let res_body = serde_json::to_string(&*res);
                match res_body {
                    Ok(r) => HttpResponse::Ok().body(r),
                    Err(e) => {
                        error!("Error serializing event : {}", e);
                        HttpResponse::BadRequest().body(e.to_string())
                    }
                }
            }
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        }
    }
}
