use core::hash::Hash;
use std::borrow::Cow;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;

use actix::{Actor, Addr, AsyncContext, Message};
use actix_web::{web, Either};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::v_models::{Player, Team, User};
use crate::services::query_services::Queriable;

// #[derive(Serialize)]
// pub struct EventQuery<'a, T: Player<'a>> {
//     pub id: Uuid,
//     pub name: Cow<'a, str>,
//     pub logo: Option<Cow<'a, str>>,
//     pub contestants: Vec<T>,
//     pub event_type: EventType,
//     pub state: EventQueryState,
// }

#[derive(Serialize)]
pub struct EventQuery<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub logo: Option<Cow<'a, str>>,
    pub event_type: EventType,
    pub state: EventQueryState,
}

pub struct EventQueryBuilder<'a> {
    pub id: Uuid,
    pub name: Cow<'a, str>,
    pub logo: Option<Cow<'a, str>>,
    pub event_type: EventType,
}
impl<'a> EventQueryBuilder<'a> {
    pub fn build_with_state(self, state: EventQueryState) -> EventQuery<'a> {
        EventQuery {
            id: self.id,
            name: self.name,
            logo: self.logo,
            event_type: self.event_type,
            state,
        }
    }
}

#[derive(Serialize)]
pub enum EventQueryState {
    Added,
    Start,
    Stop,
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
    pub count: u32,
    type_marker: PhantomData<&'a T>,
}
impl<'a, T> FtsQuery<'a, T>
where
    T: Queriable,
{
    pub fn new(count: u32, db_pool: Arc<SqlitePool>) -> Self {
        Self {
            db_pool,
            count,
            type_marker: PhantomData::<&'a T>,
        }
    }
}

pub type EitherCurEvents<'a> =
    Either<Addr<CurEventFts<'a, TeamInfo<'a>>>, Addr<CurEventFts<'a, User<'a>>>>;

#[derive(Eq, PartialEq)]
pub struct CurEventFtsWrapper<'a: 'static>(pub EitherCurEvents<'a>);

impl Hash for CurEventFtsWrapper<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.0 {
            actix_web::Either::Left(addr) => addr.hash(state),
            Either::Right(addr) => addr.hash(state),
        }
    }
}

pub struct CurFtsServer<'a: 'static> {
    pub cfts_addr: HashSet<CurEventFtsWrapper<'a>>,
}
impl CurFtsServer<'_> {
    pub fn new() -> Self {
        CurFtsServer {
            cfts_addr: HashSet::new(),
        }
    }
}
impl Actor for CurFtsServer<'_> {
    type Context = actix::Context<Self>;
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct CurFtsStop;

#[derive(Message)]
#[rtype(result = "()")]
pub struct CurFtsDisconnect<'a: 'static>(pub CurEventFtsWrapper<'a>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct CurFtsConnect<'a: 'static>(pub CurEventFtsWrapper<'a>);

#[derive(Clone, Copy)]
pub enum TeamFtsOpt {
    TeamInfo,
    RemUserInfo,
}
pub struct CurFtsBuilder<'a, P: Player<'a>> {
    event_id: Uuid,
    srv_addr: Arc<Addr<CurFtsServer<'static>>>,
    db_pool: Arc<SqlitePool>,
    type_marker: PhantomData<&'a P>,
    count: u32,
}

pub struct CurFtsTeamBuilder {
    event_id: Uuid,
    srv_addr: Arc<Addr<CurFtsServer<'static>>>,
    db_pool: Arc<SqlitePool>,
    team_opt: TeamFtsOpt,
    count: u32,
}

pub trait CurEventFtsMarker: Queriable {}
impl CurEventFtsMarker for TeamInfo<'_> {}
impl CurEventFtsMarker for User<'_> {}

pub struct CurEventFts<'a, T: CurEventFtsMarker>
where
    CurEventFts<'a, T>: Actor,
{
    pub event_id: Uuid,
    pub addr: Option<Addr<Self>>,
    pub srv_addr: Arc<Addr<CurFtsServer<'static>>>,
    pub db_pool: Arc<SqlitePool>,
    pub count: u32,
    pub team_opt: Option<TeamFtsOpt>,
    type_marker: PhantomData<&'a T>,
}

impl<'a, P> CurFtsBuilder<'a, P>
where
    P: Player<'a>,
{
    pub fn new(
        event_id: Uuid,
        srv_addr: Arc<Addr<CurFtsServer>>,
        count: u32,
        db_pool: Arc<SqlitePool>,
    ) -> Self {
        CurFtsBuilder {
            event_id,
            db_pool,
            srv_addr,
            count,
            type_marker: PhantomData::<&'a P>,
        }
    }
}

impl<'a> CurFtsBuilder<'a, User<'a>>
where
    'a: 'static,
{
    pub fn build(self) -> CurEventFts<'a, User<'a>> {
        CurEventFts {
            addr: None,
            event_id: self.event_id,
            db_pool: self.db_pool,
            srv_addr: self.srv_addr,
            team_opt: None,
            count: self.count,
            type_marker: PhantomData::<&'a User>,
        }
    }
}
impl<'a> CurFtsBuilder<'a, Team<'a>> {
    pub fn team_fts(self) -> CurFtsTeamBuilder {
        CurFtsTeamBuilder {
            event_id: self.event_id,
            srv_addr: self.srv_addr,
            db_pool: self.db_pool,
            count: self.count,
            team_opt: TeamFtsOpt::TeamInfo,
        }
    }
    pub fn rem_user_fts(self) -> CurFtsTeamBuilder {
        CurFtsTeamBuilder {
            event_id: self.event_id,
            srv_addr: self.srv_addr,
            db_pool: self.db_pool,
            count: self.count,
            team_opt: TeamFtsOpt::RemUserInfo,
        }
    }
}
impl<'a> CurFtsTeamBuilder
where
    'a: 'static,
{
    pub fn build(self) -> CurEventFts<'a, TeamInfo<'a>> {
        CurEventFts {
            addr: None,
            event_id: self.event_id,
            db_pool: self.db_pool,
            srv_addr: self.srv_addr,
            count: self.count,
            team_opt: Some(self.team_opt),
            type_marker: PhantomData::<&'a TeamInfo>,
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct VboardRes<'a>(pub Cow<'a, str>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct VbDisconnect(pub Addr<VboardClient>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct VbConnect(pub Addr<VboardClient>);

pub struct VboardClient {
    pub srv_addr: Arc<Addr<VboardSrv>>,
    pub addr: Option<Addr<Self>>,
}
impl VboardClient {
    pub fn new(srv_addr: web::Data<Addr<VboardSrv>>) -> Self {
        Self {
            srv_addr: srv_addr.into_inner(),
            addr: None,
        }
    }
}

impl Actor for VboardClient {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr = Some(addr.clone());
        self.srv_addr.do_send(VbConnect(addr))
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Some(addr) = &self.addr {
            self.srv_addr.do_send(VbDisconnect(addr.clone()))
        }
    }
}

pub struct VboardSrv {
    pub vb_addr: HashSet<Addr<VboardClient>>,
}
impl VboardSrv {
    // can also derive default instead : )
    pub fn new() -> Self {
        VboardSrv {
            vb_addr: HashSet::new(),
        }
    }
}
impl Actor for VboardSrv {
    type Context = actix::Context<Self>;
}
