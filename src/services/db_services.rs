use crate::models::v_models::{AsyncDbRes, Event, Player, Team, User, VaderEvent};
use sqlx::SqlitePool;
use uuid::Uuid;

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
    fn add_player(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        let id = self.id.to_string();
        let name = &self.name;
        let logo = self.get_logo();
        Box::pin(async move {
            sqlx::query!(
                "INSERT INTO users (id,name,score,logo) VALUES (?,?,?,?)",
                id,
                name,
                self.score,
                logo
            )
            .execute(db_pool)
            .await
            .map(|_f| ())
        })
    }
    fn update_score(&'a mut self, points: i64, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.score += points;
        let id = self.id.to_string();
        Box::pin(async move {
            sqlx::query!("UPDATE users set score=score+? WHERE id=?", points, id)
                .execute(db_pool)
                .await
                .map(|_f| ())
        })
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
    fn add_player(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        Box::pin(async move {
            let id = self.id.to_string();
            let name = &self.name;
            let logo = self.get_logo();
            sqlx::query!(
                "INSERT INTO teams (id,name,score,logo) VALUES (?,?,?,?)",
                id,
                name,
                self.score,
                logo
            )
            .execute(db_pool)
            .await
            .map(|_f| ())
        })
    }
    fn update_score(&'a mut self, points: i64, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.score += points;
        self.score += points;
        let id = self.id.to_string();
        Box::pin(async move {
            sqlx::query!("UPDATE teams set score=score+? WHERE id=?", points, id)
                .execute(db_pool)
                .await
                .map(|_f| ())
        })
    }
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}
impl<'a> Team {
    fn add_members(&'a mut self, members: Vec<User>, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.members = members.iter().map(|m| m.id).collect::<Vec<Uuid>>();
        Box::pin(async move {
            for member in &members {
                member.add_player(db_pool).await?;
            }
            let mut transaction = db_pool.begin().await?;
            let team_id = self.id.to_string();
            for member in members {
                let user_id = member.id.to_string();
                sqlx::query!(
                    "INSERT into team_users (team_id,user_id) VALUES (?,?)",
                    team_id,
                    user_id,
                )
                .execute(&mut transaction)
                .await?;
            }
            Ok(())
        })
    }
}
impl<'a> VaderEvent<'a> for Event<'a, Team> {
    type Participant = Team;
    fn add_participant(
        &self,
        participant: Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes {
        let event_id = self.id.to_string();
        let team_id = participant.id.to_string();
        Box::pin(async move {
            sqlx::query!(
                "INSERT INTO event_teams (event_id,team_id) VALUES (?,?)",
                event_id,
                team_id
            )
            .execute(db_pool)
            .await?;
            Ok(())
        })
    }

    fn add_event(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
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
            .execute(db_pool)
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
    fn add_participant(
        &'a self,
        participant: Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a> {
        let event_id = self.id.to_string();
        let user_id = participant.id.to_string();
        Box::pin(async move {
            sqlx::query!(
                "INSERT INTO event_users (event_id,user_id) VALUES (?,?)",
                event_id,
                user_id
            )
            .execute(db_pool)
            .await?;
            Ok(())
        })
    }
    fn add_event(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
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
            .execute(db_pool)
            .await
            .map(|_f| ())
        })
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}
