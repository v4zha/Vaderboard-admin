use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::v_models::{AsyncDbRes, Event, Player, Team, User, VaderEvent};

impl Player for User {
    type DeserializedType = Self;
    fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            score: 0,
        }
    }
    fn add_player(&self, db_pool: &SqlitePool) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }
    fn update_score(&mut self, points: i64, db_pool: &SqlitePool) -> AsyncDbRes {
        self.score += points;
        Box::pin(async move { unimplemented!() })
    }
    fn get_id(&self) -> Uuid {
        self.id
    }
}
impl Player for Team {
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
    fn add_player(&self, db_pool: &SqlitePool) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }
    fn update_score(&mut self, points: i64, db_pool: &SqlitePool) -> AsyncDbRes {
        self.score += points;
        Box::pin(async move { unimplemented!() })
    }
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl VaderEvent for Event<Team> {
    type Participant = Team;
    fn add_participant(&self, participant: Self::Participant) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }

    fn add_event(&self) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }
}

impl VaderEvent for Event<User> {
    type Participant = User;
    fn add_participant(&self, participant: Self::Participant) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }
    fn add_event(&self) -> AsyncDbRes {
        Box::pin(async move { unimplemented!() })
    }
}
