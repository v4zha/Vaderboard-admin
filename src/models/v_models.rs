use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

pub type AsyncDbRes = Pin<Box<dyn Future<Output = Result<(), sqlx::Error>>>>;

pub trait Player: Serialize {
    type DeserializedType: Deserialize<'static>;
    fn add_player(&self, db_pool: &SqlitePool) -> AsyncDbRes;
    fn update_score(&mut self, points: i64, db_pool: &SqlitePool) -> AsyncDbRes;
    fn new(name: String, logo: Option<String>) -> Self;
    fn get_id(&self) -> Uuid;
}

pub trait VaderEvent {
    type Participant: Player;

    fn new<T: Player>(name: String, logo: Option<String>) -> Event<T> {
        Event {
            id: Uuid::new_v4(),
            name,
            logo,
            players: Vec::<T>::new(),
        }
    }
    fn add_event(&self) -> AsyncDbRes;
    fn add_participant(&self, participant: Self::Participant) -> AsyncDbRes;
}

#[derive(Serialize, Deserialize)]
pub struct Event<T: Player> {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub logo: Option<String>,
    pub players: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<String>,
    pub members: Vec<User>,
}
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<String>,
}
