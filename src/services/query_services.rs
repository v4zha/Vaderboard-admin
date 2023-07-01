use std::marker::PhantomData;

use actix::{
    Actor, ActorContext, AsyncContext, ContextFutureSpawner, Handler, Message, StreamHandler,
    WrapFuture,
};
use actix_web_actors::ws;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqlitePool};
use uuid::Uuid;

use crate::models::error_models::VaderError;
use crate::models::query_models::{
    CurEventFts, CurEventFtsWrapper, CurFtsConnect, CurFtsDisconnect, CurFtsServer, CurFtsStop,
    EventInfo, EventQueryBuilder, EventType, FtsQuery, TeamFtsOpt, TeamInfo,
};
use crate::models::v_models::{AsyncDbRes, Event, EventState, Player, Team, User};

impl FromRow<'_, SqliteRow> for Team<'_> {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: Uuid = Uuid::parse_str(row.get("id")).map_err(|_e| sqlx::Error::ColumnDecode {
            index: "0".to_string(),
            source: Box::new(VaderError::SqlxFieldError("Error decoding Team Id")),
        })?;
        let name: String = row.get("name");
        let score: i64 = row.get("score");
        let logo: Option<String> = row.get("logo");
        let tm: String = row.get("team_members");
        let members: Vec<Uuid> = tm
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|m| {
                Uuid::parse_str(m).map_err(|_e| sqlx::Error::ColumnDecode {
                    index: "0".to_string(),
                    source: Box::new(VaderError::SqlxFieldError("Error decoding Event Id")),
                })
            })
            .collect::<Result<Vec<Uuid>, sqlx::Error>>()?;
        Ok(Team {
            id,
            name: name.into(),
            logo: logo.map(|s| s.into()),
            score,
            members,
        })
    }
}
impl FromRow<'_, SqliteRow> for User<'_> {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: Uuid = Uuid::parse_str(row.get("id")).map_err(|_e| sqlx::Error::ColumnDecode {
            index: "0".to_string(),
            source: Box::new(VaderError::SqlxFieldError("Error decoding User Id")),
        })?;
        let name: String = row.get("name");
        let score: i64 = row.get("score");
        let logo: Option<String> = row.get("logo");
        Ok(User {
            id,
            name: name.into(),
            logo: logo.map(|s| s.into()),
            score,
        })
    }
}

impl FromRow<'_, SqliteRow> for TeamInfo<'_> {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: Uuid = Uuid::parse_str(row.get("id")).map_err(|_e| sqlx::Error::ColumnDecode {
            index: "0".to_string(),
            source: Box::new(VaderError::SqlxFieldError("Error decoding User Id")),
        })?;
        let name: String = row.get("name");
        let score: i64 = row.get("score");
        let logo: Option<String> = row.get("logo");
        Ok(TeamInfo {
            id,
            name: name.into(),
            logo: logo.map(|s| s.into()),
            score,
        })
    }
}

impl<'a, T, U> FromRow<'_, SqliteRow> for Event<'a, T, U>
where
    T: Player<'a>,
    U: EventState,
{
    fn from_row(row: &'_ SqliteRow) -> Result<Event<'a, T, U>, sqlx::Error> {
        let id_str: String = row.get("id");
        let id: Uuid = Uuid::parse_str(&id_str).map_err(|_e| sqlx::Error::ColumnDecode {
            index: "0".to_string(),
            source: Box::new(VaderError::SqlxFieldError("Error decoding Event Id")),
        })?;

        let name: String = row.get("name");
        let logo: Option<String> = row.get("logo");
        let team_size: Option<u32> = row.get("team_size");

        Ok(Event {
            id,
            name: name.into(),
            logo: logo.map(|s| s.into()),
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a U>,
            team_size,
        })
    }
}
// impl<'a, U> Event<'a, Team<'a>, U>
// where
//     U: EventState,
// {
//     pub fn get_info(
//         &'a self,
//         state: EventQueryState,
//         db_pool: &'a SqlitePool,
//     ) -> AsyncDbRes<'a, EventQuery<Team>> {
//         let event_id = self.id.to_string();
//         Box::pin(async move {
//             let event = sqlx::query_as::<_, Event<'a, Team, U>>(
//                 "SELECT id,name,logo,team_size FROM events WHERE id = ? ORDER BY created_at DESC",
//             )
//             .bind(&event_id)
//             .fetch_one(db_pool)
//             .await?;
//             let contestants = sqlx::query_as::<_, Team>("SELECT t.id AS id,t.name AS name,t.score AS score,t.logo AS logo,
//                                                         GROUP_CONCAT(tm.user_id,',') AS team_members
//                                                         FROM events e JOIN event_teams et ON et.event_id = e.id
//                                                         JOIN teams t ON et.team_id=t.id
//                                                         JOIN team_members tm ON tm.team_id = t.id
//                                                         WHERE e.id = ? GROUP BY t.id")
//                 .bind(&event_id)
//                 .fetch_all(db_pool)
//                 .await?;
//             if let Some(team_size) = self.team_size {
//                 Ok(EventQuery {
//                     id: event.id,
//                     name: event.name,
//                     logo: event.logo,
//                     contestants,
//                     event_type: EventType::TeamEvent { team_size },
//                     state,
//                     marker: PhantomData::<&'a Team>,
//                 })
//             } else {
//                 Err(VaderError::TeamSizeMismatch("No Team size specified"))
//             }
//         })
//     }
// }
// impl<'a, U> Event<'a, User<'a>, U>
// where
//     U: EventState,
// {
//     pub fn get_info(
//         &'a self,
//         state: EventQueryState,
//         db_pool: &'a SqlitePool,
//     ) -> AsyncDbRes<'a, EventQuery<User>> {
//         let event_id = self.id.to_string();
//         Box::pin(async move {
//             let event = sqlx::query_as::<_, Event<'a, User, U>>(
//                 "SELECT id,name,logo,team_size FROM events WHERE id = ? ORDER BY created_at DESC",
//             )
//             .bind(&event_id)
//             .fetch_one(db_pool)
//             .await?;
//             let contestants = sqlx::query_as::<_, User>(
//                 "SELECT u.id AS id,u.name AS name,u.score AS score,u.logo AS logo
//                 FROM events e JOIN event_users eu ON eu.event_id=e.id
//                 JOIN users u ON eu.user_id=u.id
//                 WHERE e.id = ? GROUP BY u.id",
//             )
//             .bind(&event_id)
//             .fetch_all(db_pool)
//             .await?;
//
//             Ok(EventQuery {
//                 id: event.id,
//                 name: event.name,
//                 logo: event.logo,
//                 contestants,
//                 event_type: EventType::UserEvent,
//                 state,
//                 marker: PhantomData::<&'a User>,
//             })
//         })
//     }
// }
impl<'a, 'b> FromRow<'a, SqliteRow> for EventInfo<'b> {
    fn from_row(row: &'a SqliteRow) -> Result<EventInfo<'b>, sqlx::Error> {
        let id_str: String = row.get("id");
        let id: Uuid = Uuid::parse_str(&id_str).map_err(|_e| sqlx::Error::ColumnDecode {
            index: "0".to_string(),
            source: Box::new(VaderError::SqlxFieldError("Error decoding Event Id")),
        })?;
        let name: String = row.get("name");
        let logo: Option<String> = row.get("logo");
        let type_str: String = row.get("event_type");
        let team_size: Option<u32> = row.get("team_size");
        let event_type = match type_str.as_str() {
            "team_event" => match team_size {
                Some(team_size) => EventType::TeamEvent { team_size },
                None => {
                    return Err(sqlx::Error::ColumnDecode {
                        index: "0".to_string(),
                        source: Box::new(VaderError::SqlxFieldError("Error decoding EventType")),
                    });
                }
            },
            "user_event" => EventType::UserEvent,
            _ => {
                return Err(sqlx::Error::ColumnDecode {
                    index: "0".to_string(),
                    source: Box::new(VaderError::SqlxFieldError("Error decoding EventType")),
                });
            }
        };

        Ok(EventInfo {
            id,
            name: name.into(),
            logo: logo.map(|s| s.into()),
            event_type,
        })
    }
}

impl EventInfo<'_> {
    pub fn get_event_info<'a>(event_id: &'a Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, Self> {
        let id = event_id.to_string();
        Box::pin(async move {
            let event = sqlx::query_as::<_, EventInfo>(
                "SELECT id,name,logo,event_type,team_size FROM events WHERE id = ?",
            )
            .bind(&id)
            .fetch_one(db_pool)
            .await?;
            Ok(event)
        })
    }
    pub fn get_all_event_info(db_pool: &SqlitePool) -> AsyncDbRes<'_, Vec<Self>> {
        Box::pin(async move {
            let event = sqlx::query_as::<_, EventInfo>(
                "SELECT id,name,logo,event_type,team_size FROM events ORDER BY created_at DESC",
            )
            .fetch_all(db_pool)
            .await?;
            Ok(event)
        })
    }
}
impl<'a> TeamInfo<'a> {
    pub fn get_all_team_info(db_pool: &SqlitePool) -> AsyncDbRes<'_, Vec<Self>> {
        Box::pin(async move {
            let teams = sqlx::query_as::<_, TeamInfo>(
                "SELECT id,name,score,logo FROM teams ORDER BY created_at DESC",
            )
            .fetch_all(db_pool)
            .await?;
            Ok(teams)
        })
    }
    pub fn event_team_fts(
        event_id: &Uuid,
        param: &'a str,
        count: u32,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, Vec<Self>> {
        let event_id = event_id.to_string();
        Box::pin(async move {
            let teams = if param.is_empty() {
                sqlx::query_as::<_, TeamInfo>(
                    "SELECT id,name,score,logo FROM teams t 
                     JOIN event_teams et ON et.team_id=t.id 
                     WHERE et.event_id = ?
                     ORDER by t.created_at DESC 
                     LIMIT ?
                    ",
                )
                .bind(&event_id)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            } else {
                sqlx::query_as::<_, TeamInfo>(
                    "SELECT id,name,score,logo FROM teams_fts t 
                    JOIN event_teams et ON et.team_id=t.id 
                    WHERE et.event_id = ? AND name MATCH  ? || '*'
                    LIMIT ?",
                )
                .bind(&event_id)
                .bind(param)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            };
            Ok(teams)
        })
    }
    pub fn event_rem_users_fts(
        event_id: &Uuid,
        param: &'a str,
        count: u32,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, Vec<User<'a>>> {
        let event_id = event_id.to_string();
        Box::pin(async move {
            let rem_users = if param.is_empty() {
                sqlx::query_as::<_, User>(
                    "SELECT id,name,score,logo FROM users u 
                    LEFT JOIN team_members tm ON tm.user_id=u.id 
                    LEFT JOIN event_teams et ON  et.team_id=tm.team_id
                    WHERE et.event_id = ? 
                    AND tm.team_id IS NULL
                    ORDER by u.created_at DESC 
                    LIMIT ?",
                )
                .bind(&event_id)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            } else {
                sqlx::query_as::<_, User>(
                    "SELECT id,name,score,logo FROM users_fts u 
                    LEFT JOIN team_members tm ON tm.user_id=u.id 
                    LEFT JOIN event_teams et ON  et.team_id=tm.team_id
                    WHERE et.event_id = ? 
                    AND tm.team_id IS NULL
                    AND name MATCH  ? || '*'
                    LIMIT ?",
                )
                .bind(&event_id)
                .bind(param)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            };

            Ok(rem_users)
        })
    }
}

pub trait Queriable {
    type QueryRes;
    fn fts_query<'a, 'b>(
        param: &'a str,
        count: u32,
        db_pool: &'b SqlitePool,
    ) -> AsyncDbRes<'a, Vec<Self::QueryRes>>
    where
        'b: 'a;
}
impl Queriable for TeamInfo<'_> {
    type QueryRes = Self;
    fn fts_query<'a, 'b>(
        param: &'a str,
        count: u32,
        db_pool: &'b SqlitePool,
    ) -> AsyncDbRes<'a, Vec<Self::QueryRes>>
    where
        'b: 'a,
    {
        Box::pin(async move {
            let teams = if param.is_empty() {
                sqlx::query_as::<_, TeamInfo>(
                    "SELECT id,name,score,logo FROM teams t
                     ORDER by t.created_at DESC 
                     LIMIT ?",
                )
                .bind(count)
                .fetch_all(db_pool)
                .await?
            } else {
                sqlx::query_as::<_, TeamInfo>(
                    "SELECT id,name,score,logo FROM teams_fts WHERE name MATCH  ? || '*' LIMIT ?",
                )
                .bind(param)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            };
            Ok(teams)
        })
    }
}
impl Queriable for User<'_> {
    type QueryRes = Self;
    fn fts_query<'a, 'b>(
        param: &'a str,
        count: u32,
        db_pool: &'b SqlitePool,
    ) -> AsyncDbRes<'a, Vec<Self::QueryRes>>
    where
        'b: 'a,
    {
        Box::pin(async move {
            let users = if param.is_empty() {
                sqlx::query_as::<_, User>(
                    "SELECT id,name,score,logo FROM users u
                     ORDER by u.created_at DESC 
                     LIMIT ?",
                )
                .bind(count)
                .fetch_all(db_pool)
                .await?
            } else {
                sqlx::query_as::<_, User>(
                    "SELECT id,name,score,logo FROM users_fts WHERE name MATCH  ? || '*' LIMIT ?",
                )
                .bind(param)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            };

            Ok(users)
        })
    }
}
impl Queriable for EventInfo<'_> {
    type QueryRes = Self;
    fn fts_query<'a, 'b>(
        param: &'a str,
        count: u32,
        db_pool: &'b SqlitePool,
    ) -> AsyncDbRes<'a, Vec<Self::QueryRes>>
    where
        'b: 'a,
    {
        Box::pin(async move {
            let events = if param.is_empty() {
                sqlx::query_as::<_, EventInfo>(
                    "SELECT id,name,logo,event_type,team_size FROM events e
                     ORDER by e.created_at DESC 
                     LIMIT ?",
                )
                .bind(count)
                .fetch_all(db_pool)
                .await?
            } else {
                sqlx::query_as::<_, EventInfo>(
                    "SELECT id,name,logo,event_type,team_size FROM events_fts WHERE name MATCH  ? || '*' LIMIT ?",
                )
                .bind(param)
                .bind(count)
                .fetch_all(db_pool)
                .await?
            };
            Ok(events)
        })
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct FtsQueryRes(String);

impl<'a> Actor for FtsQuery<'a, TeamInfo<'_>>
where
    'a: 'static,
{
    type Context = ws::WebsocketContext<Self>;
}

impl<'a> Handler<FtsQueryRes> for FtsQuery<'a, TeamInfo<'_>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, msg: FtsQueryRes, ctx: &mut Self::Context) {
        let res_str = msg.0;
        ctx.text(res_str);
    }
}

impl<'a> StreamHandler<Result<ws::Message, ws::ProtocolError>> for FtsQuery<'a, TeamInfo<'_>>
where
    'a: 'static,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;
        let pool = self.db_pool.clone();
        let addr = ctx.address();
        let count = self.count;
        match msg {
            Ok(Ping(msg)) => ctx.pong(&msg),
            Ok(Text(param)) => {
                async move {
                    let res = TeamInfo::fts_query(&param, count, &pool)
                        .await
                        .and_then(|teams| Ok(serde_json::to_string(&teams)?));
                    match res {
                        Ok(teams) => addr.do_send(FtsQueryRes(teams)),
                        Err(e) => log::debug!("Error Getting Teams Fts : {}", e),
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}

impl<'a> Actor for FtsQuery<'a, User<'_>>
where
    'a: 'static,
{
    type Context = ws::WebsocketContext<Self>;
}

impl<'a> Handler<FtsQueryRes> for FtsQuery<'a, User<'_>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, msg: FtsQueryRes, ctx: &mut Self::Context) {
        let res_str = msg.0;
        ctx.text(res_str);
    }
}

impl<'a> StreamHandler<Result<ws::Message, ws::ProtocolError>> for FtsQuery<'a, User<'_>>
where
    'a: 'static,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;
        let pool = self.db_pool.clone();
        let addr = ctx.address();
        let count = self.count;
        match msg {
            Ok(Ping(msg)) => ctx.pong(&msg),
            Ok(Text(param)) => {
                async move {
                    let res = User::fts_query(&param, count, &pool)
                        .await
                        .and_then(|users| Ok(serde_json::to_string(&users)?));
                    match res {
                        Ok(users) => addr.do_send(FtsQueryRes(users)),
                        Err(e) => log::debug!("Error Getting Users Fts : {}", e),
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}

impl<'a> Actor for FtsQuery<'a, EventInfo<'_>>
where
    'a: 'static,
{
    type Context = ws::WebsocketContext<Self>;
}

impl<'a> Handler<FtsQueryRes> for FtsQuery<'a, EventInfo<'_>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, msg: FtsQueryRes, ctx: &mut Self::Context) {
        let res_str = msg.0;
        ctx.text(res_str);
    }
}

impl<'a> StreamHandler<Result<ws::Message, ws::ProtocolError>> for FtsQuery<'a, EventInfo<'_>>
where
    'a: 'static,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;
        let pool = self.db_pool.clone();
        let addr = ctx.address();
        let count = self.count;
        match msg {
            Ok(Ping(msg)) => ctx.pong(&msg),
            Ok(Text(param)) => {
                async move {
                    let res = EventInfo::fts_query(&param, count, &pool)
                        .await
                        .and_then(|events| Ok(serde_json::to_string(&events)?));
                    match res {
                        Ok(events_str) => addr.do_send(FtsQueryRes(events_str)),
                        Err(e) => log::debug!("Error Getting Events Fts : {}", e),
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}

impl<'a> Actor for CurEventFts<'a, TeamInfo<'a>>
where
    'a: 'static,
{
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr = Some(addr.clone());
        self.srv_addr
            .do_send(CurFtsConnect(CurEventFtsWrapper(actix_web::Either::Left(
                addr,
            ))))
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Some(addr) = &self.addr {
            self.srv_addr.do_send(CurFtsDisconnect(CurEventFtsWrapper(
                actix_web::Either::Left(addr.clone()),
            )))
        }
    }
}
impl<'a> StreamHandler<Result<ws::Message, ws::ProtocolError>> for CurEventFts<'a, TeamInfo<'a>>
where
    'a: 'static,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;
        let pool = self.db_pool.clone();
        let addr = ctx.address();
        match msg {
            Ok(Ping(msg)) => ctx.pong(&msg),
            Ok(Text(param)) => {
                let id = self.event_id;
                let team_opt = self.team_opt;
                let count = self.count;
                async move {
                    let res = match team_opt {
                        Some(ref opt) => match opt {
                            TeamFtsOpt::TeamInfo => {
                                TeamInfo::event_team_fts(&id, &param, count, &pool)
                                    .await
                                    .and_then(|teams| Ok(serde_json::to_string(&teams)?))
                            }
                            TeamFtsOpt::RemUserInfo => {
                                TeamInfo::event_rem_users_fts(&id, &param, count, &pool)
                                    .await
                                    .and_then(|users| Ok(serde_json::to_string(&users)?))
                            }
                        },
                        None => unreachable!(),
                    };

                    match res {
                        Ok(teams_str) => addr.do_send(FtsQueryRes(teams_str)),
                        Err(e) => log::debug!("Error Getting Current Event Team Fts : {}", e),
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}
impl<'a> Handler<FtsQueryRes> for CurEventFts<'a, TeamInfo<'a>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, msg: FtsQueryRes, ctx: &mut Self::Context) {
        let res_str = msg.0;
        ctx.text(res_str);
    }
}

impl<'a> Actor for CurEventFts<'a, User<'a>>
where
    'a: 'static,
{
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr = Some(addr.clone());
        self.srv_addr
            .do_send(CurFtsConnect(CurEventFtsWrapper(actix_web::Either::Right(
                addr,
            ))))
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Some(addr) = &self.addr {
            self.srv_addr.do_send(CurFtsDisconnect(CurEventFtsWrapper(
                actix_web::Either::Right(addr.clone()),
            )))
        }
    }
}
impl<'a> StreamHandler<Result<ws::Message, ws::ProtocolError>> for CurEventFts<'a, User<'a>>
where
    'a: 'static,
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        use ws::Message::*;
        let pool = self.db_pool.clone();
        let addr = ctx.address();
        let id = self.event_id;
        let team_opt = self.team_opt;
        let count = self.count;
        match msg {
            Ok(Ping(msg)) => ctx.pong(&msg),
            Ok(Text(param)) => {
                async move {
                    let res = match team_opt {
                        None => User::event_user_fts(&id, &param, count, &pool)
                            .await
                            .and_then(|users| Ok(serde_json::to_string(&users)?)),
                        Some(_) => unreachable!(),
                    };

                    match res {
                        Ok(teams_str) => addr.do_send(FtsQueryRes(teams_str)),
                        Err(e) => log::debug!("Error Getting Current Event User Fts : {}", e),
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}
impl<'a> Handler<FtsQueryRes> for CurEventFts<'a, User<'a>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, msg: FtsQueryRes, ctx: &mut Self::Context) {
        let res_str = msg.0;
        ctx.text(res_str);
    }
}

impl Handler<CurFtsStop> for CurFtsServer<'_> {
    type Result = ();
    fn handle(&mut self, _msg: CurFtsStop, _ctx: &mut Self::Context) -> Self::Result {
        self.cfts_addr
            .iter()
            .for_each(|cfts_wrapper| match &cfts_wrapper.0 {
                actix_web::Either::Left(addr) => addr.do_send(CurFtsStop),
                actix_web::Either::Right(addr) => addr.do_send(CurFtsStop),
            });
    }
}

impl Handler<CurFtsConnect<'_>> for CurFtsServer<'_> {
    type Result = ();
    fn handle(&mut self, msg: CurFtsConnect, _ctx: &mut Self::Context) -> Self::Result {
        self.cfts_addr.insert(msg.0);
    }
}

impl Handler<CurFtsDisconnect<'_>> for CurFtsServer<'_> {
    type Result = ();
    fn handle(&mut self, msg: CurFtsDisconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.cfts_addr.remove(&msg.0);
    }
}

impl<'a> Handler<CurFtsStop> for CurEventFts<'a, TeamInfo<'a>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, _msg: CurFtsStop, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(Some(ws::CloseReason {
            code: ws::CloseCode::Normal,
            description: Some("The Current Event ended".to_string()),
        }));
        ctx.stop();
    }
}
impl<'a> Handler<CurFtsStop> for CurEventFts<'a, User<'a>>
where
    'a: 'static,
{
    type Result = ();
    fn handle(&mut self, _msg: CurFtsStop, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(Some(ws::CloseReason {
            code: ws::CloseCode::Normal,
            description: Some("The Current Event ended".to_string()),
        }));
        ctx.stop();
    }
}

impl<'a, U: EventState> From<&Event<'a, Team<'a>, U>> for EventQueryBuilder<'a> {
    fn from(e: &Event<'a, Team<'a>, U>) -> Self {
        EventQueryBuilder {
            id: e.id,
            name: e.name.clone(),
            logo: e.logo.clone(),
            event_type: EventType::TeamEvent {
                team_size: e.team_size.unwrap(),
            },
        }
    }
}

impl<'a, U: EventState> From<&Event<'a, User<'a>, U>> for EventQueryBuilder<'a> {
    fn from(e: &Event<'a, User<'a>, U>) -> Self {
        EventQueryBuilder {
            id: e.id,
            name: e.name.clone(),
            logo: e.logo.clone(),
            event_type: EventType::UserEvent,
        }
    }
}
