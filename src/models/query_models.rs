use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::v_models::Player;

#[derive(Serialize)]
pub struct EventQuery<'a, T: Player<'a>> {
    pub id: Uuid,
    pub name: String,
    pub logo: Option<String>,
    pub contestants: Vec<T>,
    pub event_type: EventType,
    #[serde(skip_serializing)]
    pub marker: PhantomData<&'a T>,
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    TeamEvent,
    UserEvent,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct EventInfo {
    pub id: Uuid,
    pub name: String,
    pub logo: Option<String>,
    pub event_type: EventType,
}
