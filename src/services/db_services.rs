use crate::models::v_models::{AsyncDbRes, Event, Player, Team, User, VaderEvent};
use sqlx::SqlitePool;
use uuid::Uuid;

impl<'a> Player<'a> for Box<User> {
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
            .await?;
            Ok(())
        })
    }
    fn update_score(&'a mut self, points: i64, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.score += points;
        let id = self.id.to_string();
        Box::pin(async move {
            sqlx::query!("UPDATE users set score=score+? WHERE id=?", points, id)
                .execute(db_pool)
                .await?;
            Ok(())
        })
    }
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}

impl<'a> Player<'a> for Box<Team> {
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
            .await?;
            Ok(())
        })
    }
    fn update_score(&'a mut self, points: i64, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.score += points;
        self.score += points;
        let id = self.id.to_string();
        Box::pin(async move {
            sqlx::query!("UPDATE teams set score=score+? WHERE id=?", points, id)
                .execute(db_pool)
                .await?;
            Ok(())
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
    fn add_members(&'a mut self, members: Vec<Uuid>, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.members.extend(&members);
        Self::add_members_from_id(self.id, members, db_pool)
    }
    fn add_member(&'a mut self, member_id: Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        self.members.push(member_id);
        Self::add_member_from_id(self.id, member_id, db_pool)
    }
    fn add_member_from_id(team_id: Uuid, member: Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        let team_id = team_id.to_string();
        let user_id = member.to_string();
        Box::pin(async move {
            sqlx::query!(
                "INSERT into team_users (team_id,user_id) VALUES (?,?)",
                team_id,
                user_id,
            )
            .execute(db_pool)
            .await?;
            Ok(())
        })
    }
    fn add_members_from_id(
        team_id: Uuid,
        members: Vec<Uuid>,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a> {
        Box::pin(async move {
            let mut transaction = db_pool.begin().await?;
            let team_id = team_id.to_string();
            for mem_id in members {
                let user_id = mem_id.to_string();
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
impl<'a> Player<'a> for Box<dyn Player<'a>> {
    fn add_player(&'a self, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn update_score(&'a mut self, _points: i64, _db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn get_id(&self) -> Uuid {
        Uuid::new_v4()
    }
    fn get_logo(&self) -> String {
        String::new()
    }
}

impl<'a> VaderEvent<'a> for Event<'a, Box<Team>> {
    type Participant = Box<dyn Player<'a>>;
    fn add_participant(
        &'a self,
        participant: Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes {
        Self::add_participant_from_id(&self, participant.get_id(), db_pool)
    }
    fn add_participant_from_id(&'a self, team_id: Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes {
        let event_id = self.id.to_string();
        let team_id = team_id.to_string();
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
            .await?;
            Ok(())
        })
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}

impl<'a> VaderEvent<'a> for Event<'a, Box<User>> {
    type Participant = Box<dyn Player<'a>>;
    fn add_participant(
        &'a self,
        participant: Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a> {
        Self::add_participant_from_id(&self, participant.get_id(), db_pool)
    }
    fn add_participant_from_id(&self, user_id: Uuid, db_pool: &'a SqlitePool) -> AsyncDbRes<'a> {
        let event_id = self.id.to_string();
        let user_id = user_id.to_string();
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
            .await?;
            Ok(())
        })
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}
impl Team {
    fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            members: Vec::new(),
            score: 0,
        }
    }
}
