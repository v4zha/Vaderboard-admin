use serde::{Deserialize, Serialize};

use super::v_models::{Event, Player, Team, User, VaderEvent};

#[derive(Serialize, Deserialize)]
pub enum EventType {
    TeamEvent,
    IndividualEvent,
}

#[derive(Serialize, Deserialize)]
pub struct EventInfo {
    name: String,
    logo: Option<String>,
    event_type: EventType,
}
pub type DynEvent<'a> = Box<dyn VaderEvent<'a, Participant = Box<dyn Player<'a>>>>;

impl<'a> Into<DynEvent<'a>> for EventInfo
where
    'a: 'static,
{
    fn into(self) -> DynEvent<'a> {
        match self.event_type {
            EventType::TeamEvent => Box::new(Event::<'a, Box<Team>>::new(self.name, self.logo)),
            EventType::IndividualEvent => {
                Box::new(Event::<'a, Box<User>>::new(self.name, self.logo))
            }
        }
    }
}
