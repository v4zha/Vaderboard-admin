use std::marker::PhantomData;

use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Row, SqlitePool};
use uuid::Uuid;

use crate::models::error_models::VaderError;
use crate::models::query_models::{EventInfo, EventQuery, EventType};
use crate::models::v_models::{AsyncDbRes, Event, EventState, Player, Team, User};
impl FromRow<'_, SqliteRow> for Team {
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
            name,
            logo,
            score,
            members,
        })
    }
}
impl FromRow<'_, SqliteRow> for User {
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
            name,
            logo,
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

        Ok(Event {
            id,
            name,
            logo,
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a U>,
        })
    }
}
impl<'a, U> Event<'a, Team, U>
where
    U: EventState,
{
    pub fn get_info(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, EventQuery<Team>> {
        let event_id = self.id.to_string();
        Box::pin(async move {
            let event = sqlx::query_as::<_, Event<'a, Team, U>>(
                "SELECT id,name,logo FROM events WHERE id = ?",
            )
            .bind(&event_id)
            .fetch_one(db_pool)
            .await?;
            let contestants = sqlx::query_as::<_, Team>("SELECT t.id AS id,t.name AS name,t.score AS score,t.logo AS logo,GROUP_CONCAT(tm.user_id,',') AS team_members FROM events e JOIN event_teams et ON et.event_id = e.id JOIN teams t ON et.team_id=t.id JOIN team_members tm ON tm.team_id = t.id WHERE e.id = ? GROUP BY t.id")
                .bind(&event_id)
                .fetch_all(db_pool)
                .await?;

            Ok(EventQuery {
                id: event.id,
                name: event.name,
                logo: event.logo,
                contestants,
                event_type: EventType::TeamEvent,
                marker: PhantomData::<&'a Team>,
            })
        })
    }
}
impl<'a, U> Event<'a, User, U>
where
    U: EventState,
{
    pub fn get_info(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, EventQuery<User>> {
        let event_id = self.id.to_string();
        Box::pin(async move {
            let event = sqlx::query_as::<_, Event<'a, User, U>>(
                "SELECT id,name,logo FROM events WHERE id = ?",
            )
            .bind(&event_id)
            .fetch_one(db_pool)
            .await?;
            let contestants = sqlx::query_as::<_, User>(
                "SELECT u.name AS name,u.score AS score,u.logo AS logo FROM events e JOIN event_users eu on eu.event_id=e.id JOIN users u on eu.user_id=u.id where e.id = ? GROUP BY u.id",
            )
            .bind(&event_id)
            .fetch_all(db_pool)
            .await?;

            Ok(EventQuery {
                id: event.id,
                name: event.name,
                logo: event.logo,
                contestants,
                event_type: EventType::UserEvent,
                marker: PhantomData::<&'a User>,
            })
        })
    }
}
impl FromRow<'_, SqliteRow> for EventInfo {
    fn from_row(row: &'_ SqliteRow) -> Result<EventInfo, sqlx::Error> {
        let id_str: String = row.get("id");
        let id: Uuid = Uuid::parse_str(&id_str).map_err(|_e| sqlx::Error::ColumnDecode {
            index: "0".to_string(),
            source: Box::new(VaderError::SqlxFieldError("Error decoding Event Id")),
        })?;
        let name: String = row.get("name");
        let logo: Option<String> = row.get("logo");
        let type_str: String = row.get("event_type");
        let event_type = match type_str.as_str() {
            "team_event" => EventType::TeamEvent,
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
            name,
            logo,
            event_type,
        })
    }
}

impl EventInfo {
    pub fn get_event_info<'a>(event_id: &'a Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, Self> {
        let id = event_id.to_string();
        Box::pin(async move {
            let event = sqlx::query_as::<_, EventInfo>(
                "SELECT id,name,logo,event_type from events WHERE id = ?",
            )
            .bind(&id)
            .fetch_one(db_pool)
            .await?;
            Ok(event)
        })
    }
}
