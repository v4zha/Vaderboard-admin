use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::v_models::{AsyncDbRes, Event, Player, Team, User, VaderEvent};

impl<'a> Player<'a> for User {
    type DeserializedType = Self;
    fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            score: 0,
        }
    }
    fn add_player(&'a self, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        Box::pin(async move { unimplemented!() })
    }
    fn update_score(&'a mut self, points: i64, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.score += points;
        Box::pin(async move { unimplemented!() })
    }
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}
impl<'a> Player<'a> for Team {
    type DeserializedType = Self;
    fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            members: Vec::new(),
            score: 0,
        }
    }
    fn add_player(&'a self, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        Box::pin(async move { unimplemented!() })
    }
    fn update_score(&'a mut self, points: i64, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.score += points;
        Box::pin(async move { unimplemented!() })
    }
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}

impl<'a> VaderEvent<'a> for Event<'a, Team> {
    type Participant = Team;
    fn add_participant(&self, _participant: Self::Participant) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }

    fn add_event(&'a self, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        let logo = self.get_logo();
        let id = self.id.to_string();
        let name = &self.name;
        Box::pin(async move {
            sqlx::query!(
                "INSERT INTO events (id,name,logo,event_type) VALUES (?,?,?,?)",
                id,
                name,
                logo,
                "team_event"
            )
            .execute(_db_pool)
            .await
            .map(|_f| ())
        })
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}

impl<'a> VaderEvent<'a> for Event<'a, User> {
    type Participant = User;
    fn add_participant(&self, _participant: Self::Participant) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }
    fn add_event(&'a self, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        let logo = self.get_logo();
        let id = self.id.to_string();
        let name = &self.name;
        Box::pin(async move {
            sqlx::query!(
                "INSERT INTO events (id,name,logo,event_type) VALUES (?,?,?,?)",
                id,
                logo,
                name,
                "individual_event"
            )
            .execute(_db_pool)
            .await
            .map(|_f| ())
        })
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}
