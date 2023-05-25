use std::marker::PhantomData;

use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};
use uuid::Uuid;

use crate::models::{
    query_models::{EventQuery, EventType},
    v_models::{AsyncDbRes, Event, EventState, Player, Team, User},
};
impl FromRow<'_, SqliteRow> for Team {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: Uuid = Uuid::parse_str(row.get("id")).unwrap_or_default();
        let name: String = row.get("name");
        let score: i64 = row.get("score");
        let logo: Option<String> = row.get("logo");
        let tm: String = row.get("team_members");
        let members: Vec<Uuid> = tm
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|m| Uuid::parse_str(m).unwrap())
            .collect::<Vec<Uuid>>();
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
        let id: Uuid = Uuid::parse_str(row.get("id")).unwrap_or_default();
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
        let id_str: Option<String> = row.try_get("id")?;
        let id: Uuid = Uuid::parse_str(&id_str.unwrap()).unwrap();
        let name: Option<String> = row.try_get("name")?;
        let logo: Option<String> = row.try_get("logo").ok();

        Ok(Event {
            id,
            name: name.unwrap(),
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
                "SELECT id,name,logo,event_type from events WHERE id = ?",
            )
            .bind(&event_id)
            .fetch_one(db_pool)
            .await?;
            let contestants = sqlx::query_as::<_, Team>("")
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
                "SELECT id,name,logo,event_type from events WHERE id = ?",
            )
            .bind(&event_id)
            .fetch_one(db_pool)
            .await?;
            let contestants = sqlx::query_as::<_, User>("")
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
