use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

pub type AsyncDbRes = Pin<Box<dyn Future<Output = Result<(), sqlx::Error>>>>;
pub trait Player: Serialize {
    type DeserializedType: Deserialize<'static>;
    fn add_player(&self, db_pool: &SqlitePool) -> AsyncDbRes;
    fn update_score(&self, db_pool: &SqlitePool) -> AsyncDbRes;
}

#[derive(Serialize, Deserialize)]
pub struct Event<T: Player> {
    id: Uuid,
    name: String,
    #[serde(default)]
    logo: Option<String>,
    players: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    id: Uuid,
    name: String,
    score: i64,
    #[serde(default)]
    logo: Option<String>,
    #[serde(default)]
    desc: Option<String>,
    members: Vec<User>,
}
#[derive(Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    #[serde(default)]
    logo: Option<String>,
    name: String,
    #[serde(default)]
    score: i64,
}
