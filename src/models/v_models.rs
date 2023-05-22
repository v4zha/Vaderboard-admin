use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::error_models::VaderError;

pub type AsyncDbRes<'a, T> = Pin<Box<dyn Future<Output = Result<T, VaderError>> + Send + 'a>>;

pub enum EventStateWrapper<'a, T: Player<'a>> {
    New(Event<'a, T, NewEvent>),
    Active(Event<'a, T, ActiveEvent>),
    End(Event<'a, T, EndEvent>),
}

impl<'a, T> EventStateWrapper<'a, T>
where
    T: Player<'a>,
{
    fn start_event(&mut self) {
        match self {
            Self::New(event) => *self = Self::Active(event.start_event()),
            _ => {}
        }
    }
    fn end_event(&mut self) {
        match self {
            Self::Active(event) => *self = Self::End(event.end_event()),
            _ => {}
        }
    }
    fn get_id(&self) -> Uuid {
        match self {
            Self::New(e) => e.id,
            Self::Active(e) => e.id,
            Self::End(e) => e.id,
        }
    }
}
pub enum EventWrapper<'a> {
    TeamEvent(EventStateWrapper<'a, Team>),
    UserEvent(EventStateWrapper<'a, User>),
}
impl<'a> EventWrapper<'a> {
    pub fn start_event(&mut self) {
        match self {
            Self::TeamEvent(sw) => sw.start_event(),
            Self::UserEvent(sw) => sw.start_event(),
        }
    }
    pub fn end_event(&mut self) {
        match self {
            Self::TeamEvent(sw) => sw.end_event(),
            Self::UserEvent(sw) => sw.end_event(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Self::TeamEvent(sw) => sw.get_id(),
            Self::UserEvent(sw) => sw.get_id(),
        }
    }
    pub fn update_score_by_id(
        &self,
        p_id: &Uuid,
        score: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        match self {
            Self::TeamEvent(sw) => match sw {
                EventStateWrapper::Active(e) => e.update_score_by_id(p_id, score, db_pool),
                _ => Box::pin(async move {
                    Err(VaderError::EventNotActive(
                        "Event is not active to Update Score".to_string(),
                    ))
                }),
            },
            Self::UserEvent(sw) => match sw {
                EventStateWrapper::Active(e) => e.update_score_by_id(p_id, score, db_pool),
                _ => Box::pin(async move {
                    Err(VaderError::EventNotActive(
                        "Event is not active to Update Score".to_string(),
                    ))
                }),
            },
        }
    }
}
pub struct AppState {
    pub current_event: Arc<Mutex<Option<EventWrapper<'static>>>>,
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            current_event: Arc::new(Mutex::new(None)),
        }
    }
}

pub trait Player<'a> {
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
    fn add_participant_from_id(
        team_id: Uuid,
        p_id: Uuid,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()>;
    fn get_logo(&self) -> String;
}

pub trait EventState {}

pub struct NewEvent;
pub struct ActiveEvent;
pub struct EndEvent;
impl EventState for NewEvent {}
impl EventState for ActiveEvent {}
impl EventState for EndEvent {}

#[derive(Serialize, Deserialize)]
pub struct Event<'a, T: Player<'a>, U: EventState = NewEvent> {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub logo: Option<String>,
    pub player_marker: PhantomData<&'a T>,
    pub state_marker: PhantomData<&'a U>,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<String>,
    pub members: Vec<Uuid>,
}
#[derive(Serialize, Deserialize)]
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
