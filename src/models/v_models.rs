use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use uuid::Uuid;

pub type AsyncDbRes<'a, T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send + 'a>>;

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

pub struct ActiveEvent;
pub struct NewEvent;
pub struct EndEvent;
impl EventState for ActiveEvent {}
impl EventState for NewEvent {}
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
