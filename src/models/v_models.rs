use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error_models::VaderError;
use super::wrapper_models::EventWrapper;

pub type AsyncDbRes<'a, T> = Pin<Box<dyn Future<Output = Result<T, VaderError<'a>>> + Send + 'a>>;

pub struct AppState {
    pub current_event: Mutex<Option<EventWrapper<'static>>>,
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            current_event: Mutex::new(None),
        }
    }
}

pub trait Player<'a>: Send + Sync {
    fn add_player(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()>;
    fn get_id(&self) -> Uuid;
    fn get_logo(&self) -> String;
}

pub trait VaderEvent<'a> {
    type Participant: Player<'a>;
    fn add_event(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()>;
    fn add_participant(
        &'a self,
        participant: &Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()>;
    fn add_participant_from_id(&'a self, p_id: Uuid, db_pool: &'a SqlitePool)
        -> AsyncDbRes<'a, ()>;
    fn get_logo(&self) -> String;
}

pub trait EventState: Send + Sync {}

#[derive(Debug)]
pub struct NewEvent;
#[derive(Debug)]
pub struct ActiveEvent;
#[derive(Debug)]
pub struct EndEvent;
impl EventState for NewEvent {}
impl EventState for ActiveEvent {}
impl EventState for EndEvent {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event<'a, T: Player<'a>, U: EventState = NewEvent> {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub logo: Option<String>,
    pub player_marker: PhantomData<&'a T>,
    pub state_marker: PhantomData<&'a U>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<String>,
    pub members: Vec<Uuid>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<String>,
}

impl<'a, T: Player<'a>, U: EventState> Event<'a, T, U> {
    pub fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a U>,
        }
    }
}
