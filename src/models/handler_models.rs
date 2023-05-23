use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::v_models::{Event, Team, User};

#[derive(Deserialize)]
pub enum EventType {
    TeamEvent,
    UserEvent,
}
#[derive(Deserialize)]
pub struct EventInfo {
    name: String,
    logo: Option<String>,
    pub event_type: EventType,
}
impl Into<Event<'_, Team>> for EventInfo {
    fn into(self) -> Event<'static, Team> {
        Event::<Team>::new(self.name, self.logo)
    }
}
impl Into<Event<'_, User>> for EventInfo {
    fn into(self) -> Event<'static, User> {
        Event::<User>::new(self.name, self.logo)
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

impl Into<User> for ContestantInfo {
    fn into(self) -> User {
        User::new(self.name, self.logo)
    }
}
impl Into<Team> for ContestantInfo {
    fn into(self) -> Team {
        Team::new(self.name, self.logo)
    }
}

#[derive(Serialize)]
pub struct CreationResponse<'a> {
    msg: &'a str,
    id: Uuid,
}
impl<'a> CreationResponse<'a> {
    pub fn new(msg: &'a str, id: Uuid) -> Self {
        Self { msg, id }
    }
}
#[derive(Deserialize)]
pub struct MemberInfo {
    pub team_id: Uuid,
    pub members: Vec<Uuid>,
}
