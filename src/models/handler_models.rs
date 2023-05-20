use serde::{Deserialize, Serialize};

use super::v_models::{Event, Team, User};

#[derive(Serialize, Deserialize)]
pub struct EventInfo {
    name: String,
    logo: Option<String>,
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
