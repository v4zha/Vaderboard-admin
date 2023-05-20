use serde::{Deserialize, Serialize};
use sqlx::{Error, SqlitePool};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use uuid::Uuid;

pub type AsyncDbRes<'a, T> = Pin<Box<dyn Future<Output = Result<T, Error>> + Send + 'a>>;

pub trait Player<'a> {
    fn add_player(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()>;
    fn update_score(&'a mut self, points: i64, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()>;
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

#[derive(Serialize, Deserialize)]
pub struct Event<'a, T: Player<'a>> {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub logo: Option<String>,
    marker: PhantomData<&'a T>,
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

impl<'a, T: Player<'a>> Event<'a, T> {
    pub fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            marker: PhantomData,
        }
    }
}
