use std::borrow::Cow;
use std::marker::PhantomData;
use std::sync::Arc;

use actix::{Actor, Message};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::v_models::{Player, Team, User};
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
    TeamEvent { team_size: u32 },
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
#[derive(Clone)]
pub enum TeamFtsOpt {
    TeamInfo,
    RemUserInfo,
}
pub struct CurFtsBuilder<'a, P: Player<'a>> {
    event_id: Uuid,
    db_pool: Arc<SqlitePool>,
    type_marker: PhantomData<&'a P>,
}

pub struct CurFtsTeamBuilder {
    event_id: Uuid,
    db_pool: Arc<SqlitePool>,
    team_opt: TeamFtsOpt,
}

impl<'a, P> CurFtsBuilder<'a, P>
where
    P: Player<'a>,
{
    pub fn new(event_id: Uuid, db_pool: Arc<SqlitePool>) -> Self {
        CurFtsBuilder {
            event_id,
            db_pool,
            type_marker: PhantomData::<&'a P>,
        }
    }
}

impl<'a> CurFtsBuilder<'a, User<'a>> {
    pub fn build(self) -> CurEventFts<'a, User<'a>> {
        CurEventFts {
            event_id: self.event_id,
            db_pool: self.db_pool,
            team_opt: None,
            type_marker: PhantomData::<&'a User>,
        }
    }
}
impl<'a> CurFtsBuilder<'a, Team<'a>> {
    pub fn team_fts(self) -> CurFtsTeamBuilder {
        CurFtsTeamBuilder {
            event_id: self.event_id,
            db_pool: self.db_pool,
            team_opt: TeamFtsOpt::TeamInfo,
        }
    }
    pub fn rem_user_fts(self) -> CurFtsTeamBuilder {
        CurFtsTeamBuilder {
            event_id: self.event_id,
            db_pool: self.db_pool,
            team_opt: TeamFtsOpt::RemUserInfo,
        }
    }
}
impl<'a> CurFtsTeamBuilder {
    pub fn build(self) -> CurEventFts<'a, TeamInfo<'a>> {
        CurEventFts {
            event_id: self.event_id,
            db_pool: self.db_pool,
            team_opt: Some(self.team_opt),
            type_marker: PhantomData::<&'a TeamInfo>,
        }
    }
}

pub struct CurEventFts<'a, T: Queriable> {
    pub event_id: Uuid,
    pub db_pool: Arc<SqlitePool>,
    pub team_opt: Option<TeamFtsOpt>,
    type_marker: PhantomData<&'a T>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct VboardRes<'a>(pub Cow<'a, str>);

pub struct Vboard {}

impl Actor for Vboard {
    type Context = ws::WebsocketContext<Self>;
}
