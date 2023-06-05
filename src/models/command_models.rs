use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error_models::VaderError;
use super::query_models::EventType;
use super::v_models::{Event, Team, User};

#[derive(Deserialize, Serialize)]
pub struct EventReq<'a> {
    name: Cow<'a, str>,
    #[serde(default)]
    logo: Option<Cow<'a, str>>,
    pub event_type: EventType,
}

impl<'a> From<EventReq<'a>> for Result<Event<'a, Team<'a>>, VaderError<'a>> {
    fn from(req: EventReq<'a>) -> Self {
        match req.event_type {
            EventType::TeamEvent { team_size } => {
                Ok(Event::<Team>::new(req.name, req.logo, Some(team_size)))
            }
            EventType::UserEvent => {
                Err(VaderError::TeamSizeMismatch("time size was not specified"))
            }
        }
    }
}
impl<'a> From<EventReq<'a>> for Result<Event<'a, User<'a>>, VaderError<'a>> {
    fn from(req: EventReq<'a>) -> Self {
        Ok(Event::<User>::new(req.name, req.logo, None))
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
#[derive(Serialize)]
pub struct ScoreResponse {
    id: Uuid,
    new_score: i32,
}
impl ScoreResponse {
    pub fn new(id: Uuid, new_score: i32) -> Self {
        Self { id, new_score }
    }
}
