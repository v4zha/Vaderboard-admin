use sqlx::SqlitePool;

use crate::models::v_models::{AsyncDbRes, Player, Team, User};

impl Player for User {
    type DeserializedType = Self;
    fn add_player(&self, db_pool: &SqlitePool) -> AsyncDbRes {
        Box::pin(async move {
            unimplemented!()
        })
    }
    fn update_score(&self, db_pool: &SqlitePool) -> AsyncDbRes {
        Box::pin(async move {})
            unimplemented!()
    }
}
impl Player for Team {
    type DeserializedType = Self;
    fn add_player(&self, db_pool: &SqlitePool) -> AsyncDbRes {
        Box::pin(async move {})
            unimplemented!()
    }
    fn update_score(&self, db_pool: &SqlitePool) -> AsyncDbRes {
        Box::pin(async move {})
            unimplemented!()
    }
}
