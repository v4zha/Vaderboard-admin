use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::query_models::EventType;
use super::v_models::{Event, Team, User};

#[derive(Deserialize, Serialize)]
pub struct EventReq {
    name: String,
    #[serde(default)]
    logo: Option<String>,
    pub event_type: EventType,
}
impl From<EventReq> for Event<'_, Team> {
    fn from(req: EventReq) -> Self {
        Event::<Team>::new(req.name, req.logo)
    }
}
impl From<EventReq> for Event<'_, User> {
    fn from(req: EventReq) -> Self {
        Event::<User>::new(req.name, req.logo)
    }
}
#[derive(Deserialize)]
pub struct ScoreUpdate {
    pub id: Uuid,
    pub score: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ContestantInfo {
    name: String,
    #[serde(default)]
    logo: Option<String>,
}

impl From<ContestantInfo> for User {
    fn from(ci: ContestantInfo) -> User {
        User::new(ci.name, ci.logo)
    }
}
impl From<ContestantInfo> for Team {
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
