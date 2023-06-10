use std::borrow::Cow;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use bcrypt::verify;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tokio::sync::Mutex;
use uuid::Uuid;

use super::error_models::VaderError;
use super::wrapper_models::EventWrapper;

pub type AsyncDbRes<'a, T> = Pin<Box<dyn Future<Output = Result<T, VaderError<'a>>> + Send + 'a>>;

pub struct AppState {
    pub current_event: Mutex<Option<EventWrapper<'static>>>,
    pub vb_count: u32,
}
impl AppState {
    pub fn new(vb_count: u32) -> Self {
        AppState {
            current_event: Mutex::new(None),
            vb_count,
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

pub struct NewEvent;
pub struct ActiveEvent;
pub struct EndEvent;
impl EventState for NewEvent {}
impl EventState for ActiveEvent {}
impl EventState for EndEvent {}

#[derive(Serialize, Deserialize)]
pub struct Event<'a, T: Player<'a>, U: EventState = NewEvent> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    #[serde(default)]
    pub logo: Option<Cow<'a, str>>,
    pub team_size: Option<u32>,
    pub player_marker: PhantomData<&'a T>,
    pub state_marker: PhantomData<&'a U>,
}

#[derive(Deserialize, FromRow)]
pub struct AdminInfo {
    pub username: String,
    pub password: String,
}
impl AdminInfo {
    //Never use in production : )
    // Use an auth service , athakum nallath : )
    // also use argon2 for hashing as alternative to bcrypt
    pub fn verify_passwd(self, db_pool: &SqlitePool) -> AsyncDbRes<'_, bool> {
        Box::pin(async move {
            let res = sqlx::query_as::<_, Self>(
                "SELECT username,password FROM admin_login WHERE username = ?",
            )
            .bind(&self.username)
            .fetch_one(db_pool)
            .await?;
            let verify_res =
                actix_web::web::block(move || verify(self.password, &res.password)).await??;
            Ok(verify_res)
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct Team<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<Cow<'a, str>>,
    pub members: Vec<Uuid>,
}
#[derive(Serialize, Deserialize)]
pub struct User<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub score: i64,
    #[serde(default)]
    pub logo: Option<Cow<'a, str>>,
}

impl<'a, T: Player<'a>, U: EventState> Event<'a, T, U> {
    pub fn new(name: Cow<'a, str>, logo: Option<Cow<'a, str>>, team_size: Option<u32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            team_size,
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a U>,
        }
    }
    pub fn delete_event(id: &'a Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
        let id = id.to_string();
        Box::pin(async move {
            let res = sqlx::query!("DELETE FROM events WHERE id = ? ", id)
                .execute(db_pool)
                .await?;
            if res.rows_affected().eq(&0) {
                return Err(VaderError::EventNotFound("No event found"));
            }
            Ok(())
        })
    }
}
