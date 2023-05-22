use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::v_models::{Event, Team, User};

#[derive(Serialize, Deserialize)]
pub enum EventType {
    TeamEvent,
    UserEvent,
}
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub struct ScoreUpdate {
    pub id: Uuid,
    pub score: i64,
}
