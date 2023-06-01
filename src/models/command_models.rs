use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::query_models::EventType;
use super::v_models::{Event, Team, User};

#[derive(Deserialize, Serialize)]
pub struct EventReq<'a> {
    name: Cow<'a, str>,
    #[serde(default)]
    logo: Option<Cow<'a, str>>,
    pub event_type: EventType,
}
impl<'a> From<EventReq<'a>> for Event<'a, Team<'a>> {
    fn from(req: EventReq<'a>) -> Self {
        Event::<Team>::new(req.name, req.logo)
    }
}
impl<'a> From<EventReq<'a>> for Event<'a, User<'a>> {
    fn from(req: EventReq<'a>) -> Self {
        Event::<User>::new(req.name, req.logo)
    }
}
#[derive(Deserialize)]
pub struct ScoreUpdate {
    pub id: Uuid,
    pub score: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ContestantInfo<'a> {
    name: Cow<'a, str>,
    #[serde(default)]
    logo: Option<Cow<'a, str>>,
}

impl<'a> From<ContestantInfo<'a>> for User<'a> {
    fn from(ci: ContestantInfo) -> User {
        User::new(ci.name, ci.logo)
    }
}
impl<'a> From<ContestantInfo<'a>> for Team<'a> {
    fn from(ci: ContestantInfo) -> Team {
        Team::new(ci.name, ci.logo)
    }
}

#[derive(Serialize)]
pub struct CommandResponse<'a> {
    msg: &'a str,
    id: Uuid,
}
impl<'a> CommandResponse<'a> {
    pub fn new(msg: &'a str, id: Uuid) -> Self {
        Self { msg, id }
    }
}
#[derive(Deserialize)]
pub struct MemberInfo {
    pub team_id: Uuid,
    pub members: Vec<Uuid>,
}
