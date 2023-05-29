use sqlx::SqlitePool;
use uuid::Uuid;

use super::{
    command_models::MemberInfo,
    error_models::VaderError,
    v_models::{
        ActiveEvent, AsyncDbRes, EndEvent, Event, NewEvent, Player, Team, User, VaderEvent,
    },
};
use erased_serde::Serialize as ErasedSerialize;

pub enum EventStateWrapper<'a, T: Player<'a>> {
    New(Event<'a, T, NewEvent>),
    Active(Event<'a, T, ActiveEvent>),
    End(Event<'a, T, EndEvent>),
}

impl<'a, T> EventStateWrapper<'a, T>
where
    T: Player<'a>,
{
    fn start_event(&mut self) -> Result<(), VaderError> {
        match self {
            Self::New(event) => {
                *self = Self::Active(event.start_event());
                Ok(())
            }
            Self::Active(_) => Err(VaderError::EventActive("Event already Started")),
            Self::End(_) => Err(VaderError::EventEnded("Event already Ended")),
        }
    }
    fn end_event(&mut self) -> Result<(), VaderError> {
        match self {
            Self::Active(event) => {
                *self = Self::End(event.end_event());
                Ok(())
            }
            Self::New(_) => Err(VaderError::EventNotActive("Event didn't start")),
            Self::End(_) => Err(VaderError::EventEnded("Event already Ended")),
        }
    }
    fn get_id(&self) -> Uuid {
        match self {
            Self::New(e) => e.id,
            Self::Active(e) => e.id,
            Self::End(e) => e.id,
        }
    }
}
pub enum EventWrapper<'a> {
    TeamEvent(EventStateWrapper<'a, Team>),
    UserEvent(EventStateWrapper<'a, User>),
}
impl<'a> EventWrapper<'a> {
    pub fn start_event(&mut self) -> Result<(), VaderError> {
        match self {
            Self::TeamEvent(sw) => sw.start_event(),
            Self::UserEvent(sw) => sw.start_event(),
        }
    }
    pub fn end_event(&mut self) -> Result<(), VaderError> {
        match self {
            Self::TeamEvent(sw) => sw.end_event(),
            Self::UserEvent(sw) => sw.end_event(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        match self {
            Self::TeamEvent(sw) => sw.get_id(),
            Self::UserEvent(sw) => sw.get_id(),
        }
    }
    pub fn update_score_by_id(
        &self,
        p_id: &Uuid,
        score: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        match self {
            Self::TeamEvent(sw) => match sw {
                EventStateWrapper::Active(e) => e.update_score_by_id(p_id, score, db_pool),
                _ => Box::pin(async move {
                    Err(VaderError::EventNotActive(
                        "Event is not active to Update Score",
                    ))
                }),
            },
            Self::UserEvent(sw) => match sw {
                EventStateWrapper::Active(e) => e.update_score_by_id(p_id, score, db_pool),
                _ => Box::pin(async move {
                    Err(VaderError::EventNotActive(
                        "Event is not active to Update Score",
                    ))
                }),
            },
        }
    }
    pub fn reset_score(&self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
        match self {
            Self::TeamEvent(sw) => match sw {
                EventStateWrapper::New(e) => e.reset_score(db_pool),
                _ => Box::pin(async move {
                    Err(VaderError::EventTypeMismatch(
                        "Unable to reset score , Event may have already started / ended.",
                    ))
                }),
            },
            Self::UserEvent(sw) => match sw {
                EventStateWrapper::New(e) => e.reset_score(db_pool),
                _ => Box::pin(async move {
                    Err(VaderError::EventTypeMismatch(
                        "Event is not active to Update Score",
                    ))
                }),
            },
        }
    }
    pub fn add_team(&self, team: Team, db_pool: &'a SqlitePool) -> AsyncDbRes<()> {
        match self {
            Self::TeamEvent(sw) => match sw {
                EventStateWrapper::New(e) => Box::pin(async move {
                    let _ = team.add_player(db_pool).await;
                    e.add_participant_from_id(team.id, db_pool).await
                }),
                _ => Box::pin(async move {
                    Err(VaderError::EventActive(
                        "Team cannot be added as Event already started",
                    ))
                }),
            },
            Self::UserEvent(_) => Box::pin(async move {
                Err(VaderError::EventTypeMismatch(
                    "Cannot add team in user event",
                ))
            }),
        }
    }
    pub fn add_team_members(&self, mi: &'a MemberInfo, db_pool: &'a SqlitePool) -> AsyncDbRes<()> {
        match self {
            Self::TeamEvent(sw) => {
                match sw {
                    EventStateWrapper::New(e) => Box::pin(async move {
                        e.add_team_members(&mi.team_id, &mi.members, db_pool).await
                    }),
                    _ => Box::pin(async move {
                        Err(VaderError::EventActive(
                            "TeamMembers cannot be added as Event already started",
                        ))
                    }),
                }
            }
            Self::UserEvent(_) => Box::pin(async move {
                Err(VaderError::EventTypeMismatch(
                    "Cannot add teamMember in user event",
                ))
            }),
        }
    }
    pub fn add_user(&self, user: &'a User, db_pool: &'a SqlitePool) -> AsyncDbRes<()> {
        match self {
            Self::TeamEvent(sw) => match sw {
                EventStateWrapper::New(_) => {
                    Box::pin(async move { user.add_player(db_pool).await })
                }
                _ => Box::pin(async move {
                    Err(VaderError::EventActive(
                        "User cannot be added as Event already started",
                    ))
                }),
            },
            Self::UserEvent(sw) => match sw {
                EventStateWrapper::New(e) => Box::pin(async move {
                    let _ = user.add_player(db_pool).await;
                    e.add_participant_from_id(user.id, db_pool).await
                }),
                _ => Box::pin(async move {
                    Err(VaderError::EventActive(
                        "User cannot be added as Event already started",
                    ))
                }),
            },
        }
    }
    pub fn get_event(
        &'a self,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<Box<dyn ErasedSerialize + 'a>> {
        match self {
            Self::TeamEvent(sw) => match sw {
                EventStateWrapper::New(e) => Box::pin(async move {
                    let event = e.get_info(db_pool).await?;
                    Ok(Box::new(event) as Box<dyn ErasedSerialize>)
                }),
                EventStateWrapper::Active(e) => Box::pin(async move {
                    let event = e.get_info(db_pool).await?;
                    Ok(Box::new(event) as Box<dyn ErasedSerialize>)
                }),
                EventStateWrapper::End(e) => Box::pin(async move {
                    let event = e.get_info(db_pool).await?;
                    Ok(Box::new(event) as Box<dyn ErasedSerialize>)
                }),
            },
            Self::UserEvent(sw) => match sw {
                EventStateWrapper::New(e) => Box::pin(async move {
                    let event = e.get_info(db_pool).await?;
                    Ok(Box::new(event) as Box<dyn ErasedSerialize>)
                }),
                EventStateWrapper::Active(e) => Box::pin(async move {
                    let event = e.get_info(db_pool).await?;
                    Ok(Box::new(event) as Box<dyn ErasedSerialize>)
                }),
                EventStateWrapper::End(e) => Box::pin(async move {
                    let event = e.get_info(db_pool).await?;
                    Ok(Box::new(event) as Box<dyn ErasedSerialize>)
                }),
            },
        }
    }
}