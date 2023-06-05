use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;

use actix::{Actor, Message};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::v_models::Player;
use crate::services::query_services::Queriable;

#[derive(Serialize)]
pub struct EventQuery<'a, T: Player<'a>> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub logo: Option<Cow<'a, str>>,
    pub contestants: Vec<T>,
    pub event_type: EventType,
    #[serde(skip_serializing)]
    pub marker: PhantomData<&'a T>,
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    //u32 -> team size
    TeamEvent(u32),
    UserEvent,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct EventInfo<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub logo: Option<Cow<'a, str>>,
    pub event_type: EventType,
}

#[derive(Serialize)]
pub struct TeamInfo<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub score: i64,
    pub logo: Option<Cow<'a, str>>,
}

pub struct FtsQuery<'a, T: Queriable> {
    pub db_pool: Arc<SqlitePool>,
    type_marker: PhantomData<&'a T>,
}
impl<'a, T> FtsQuery<'a, T>
where
    T: Queriable,
{
    pub fn new(db_pool: Arc<SqlitePool>) -> Self {
        Self {
            db_pool,
            type_marker: PhantomData::<&'a T>,
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct VboardRes<'a>(pub Cow<'a, str>);

pub struct Vboard {}

impl Actor for Vboard {
    type Context = ws::WebsocketContext<Self>;
}
