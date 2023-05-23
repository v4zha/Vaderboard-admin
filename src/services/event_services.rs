use std::marker::PhantomData;

use crate::models::{
    error_models::VaderError,
    v_models::{
        ActiveEvent, AsyncDbRes, EndEvent, Event, NewEvent, Player, Team, User, VaderEvent,
    },
};
use sqlx::SqlitePool;
use uuid::Uuid;

impl<'a> Player<'a> for User {
    fn add_player(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
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
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}

impl<'a> Player<'a> for Team {
    fn add_player(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
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
    fn get_id(&self) -> Uuid {
        self.id
    }
    fn get_logo(&self) -> String {
        self.logo.as_ref().unwrap_or(&String::new()).to_string()
    }
}
impl<'a> Team {
    fn add_members_from_id(
        team_id: &'a Uuid,
        members: &'a [Uuid],
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        Box::pin(async move {
            let mut transaction = db_pool.begin().await?;
            let team_id = team_id.to_string();
            for mem_id in members {
                let user_id = mem_id.to_string();
                let res = sqlx::query!(
                    "INSERT into team_members (team_id,user_id) VALUES (?,?)",
                    team_id,
                    user_id,
                )
                .execute(&mut transaction)
                .await;
                match res {
                    Ok(c) => {
                        if c.rows_affected().eq(&0) {
                            return Err(VaderError::TeamNotFound(
                                "No Team found to Add Team Members",
                            ));
                        }
                    }
                    Err(err) => {
                        log::error!("Unable to add member :  {}", mem_id);
                        return Err(VaderError::SqlxError(err));
                    }
                }
            }
            Ok(())
        })
    }
}
impl<'a> VaderEvent<'a> for Event<'a, Team> {
    type Participant = Team;
    fn add_participant(
        &'a self,
        participant: &Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        Self::add_participant_from_id(self, participant.get_id(), db_pool)
    }
    fn add_participant_from_id(
        &'a self,
        team_id: Uuid,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
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

    fn add_event(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
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

impl<'a> VaderEvent<'a> for Event<'a, User> {
    type Participant = User;
    fn add_participant(
        &'a self,
        participant: &Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        Self::add_participant_from_id(&self, participant.get_id(), db_pool)
    }
    fn add_participant_from_id(
        &'a self,
        user_id: Uuid,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
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
    fn add_event(&'a self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
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

impl<'a> Event<'a, User, ActiveEvent> {
    pub fn update_score(
        &self,
        user: &User,
        points: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        let id = user.id;
        Self::update_score_by_id(self, &id, points, db_pool)
    }
    pub fn update_score_by_id(
        &self,
        user_id: &Uuid,
        points: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        let id = user_id.to_string();
        Box::pin(async move {
            let res = sqlx::query!("UPDATE users set score=score+? WHERE id=?", points, id)
                .execute(db_pool)
                .await;
            match res {
                Ok(c) => {
                    if c.rows_affected().eq(&0) {
                        return Err(VaderError::UserNotFound("No User Found to update Score"));
                    }
                }
                Err(err) => return Err(VaderError::SqlxError(err)),
            }
            Ok(())
        })
    }
}

impl<'a> Event<'a, Team> {
    pub fn add_team_members(
        &'a self,
        team_id: &'a Uuid,
        members: &'a [Uuid],
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<()> {
        Team::add_members_from_id(team_id, members, db_pool)
    }
}

impl<'a> Event<'a, Team, ActiveEvent> {
    pub fn update_score(
        &self,
        team: &Team,
        points: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        let id = team.id;
        Self::update_score_by_id(self, &id, points, db_pool)
    }
    pub fn update_score_by_id(
        &self,
        team_id: &Uuid,
        points: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        let id = team_id.to_string();
        Box::pin(async move {
            let res = sqlx::query!("UPDATE teams set score=score+? WHERE id=?", points, id)
                .execute(db_pool)
                .await;
            match res {
                Ok(c) => {
                    if c.rows_affected().eq(&0) {
                        return Err(VaderError::TeamNotFound("No Team Found to update Score"));
                    }
                }
                Err(err) => return Err(VaderError::SqlxError(err)),
            }
            Ok(())
        })
    }
}
impl<'a, T> Event<'a, T, NewEvent>
where
    T: Player<'a>,
{
    pub fn start_event(&self) -> Event<'a, T, ActiveEvent> {
        Into::<Event<'a, T, ActiveEvent>>::into(self)
    }
}
impl<'a, T> Event<'a, T, ActiveEvent>
where
    T: Player<'a>,
{
    pub fn end_event(&self) -> Event<'a, T, EndEvent> {
        Into::<Event<'a, T, EndEvent>>::into(self)
    }
}

impl<'a, T> Into<Event<'a, T, ActiveEvent>> for &Event<'a, T, NewEvent>
where
    T: Player<'a>,
{
    fn into(self) -> Event<'a, T, ActiveEvent> {
        Event {
            id: self.id,
            name: self.name.to_owned(),
            logo: self.logo.to_owned(),
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a ActiveEvent>,
        }
    }
}
impl<'a, T> Into<Event<'a, T, EndEvent>> for &Event<'a, T, ActiveEvent>
where
    T: Player<'a>,
{
    fn into(self) -> Event<'a, T, EndEvent> {
        Event {
            id: self.id,
            name: self.name.to_owned(),
            logo: self.logo.to_owned(),
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a EndEvent>,
        }
    }
}

impl Team {
    pub fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            members: Vec::new(),
            score: 0,
        }
    }
}
impl User {
    pub fn new(name: String, logo: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            score: 0,
        }
    }
}
