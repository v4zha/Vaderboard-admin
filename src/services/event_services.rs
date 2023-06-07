use std::borrow::Cow;
use std::marker::PhantomData;

use sqlx::SqlitePool;
use uuid::Uuid;

use super::query_services::Queriable;
use crate::models::error_models::VaderError;
use crate::models::query_models::TeamInfo;
use crate::models::v_models::{
    ActiveEvent, AsyncDbRes, EndEvent, Event, EventState, NewEvent, Player, Team, User, VaderEvent,
};

impl<'a> Player<'a> for User<'a> {
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
        self.logo.clone().unwrap_or(Cow::Borrowed("")).to_string()
    }
}

impl<'a> Player<'a> for Team<'a> {
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
        self.logo.clone().unwrap_or(Cow::Borrowed("")).to_string()
    }
}
impl<'a> Team<'a> {
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
                    "INSERT INTO team_members (team_id,user_id) VALUES (?,?)",
                    team_id,
                    user_id,
                )
                .execute(&mut transaction)
                .await;
                match res {
                    Ok(c) => {
                        if c.rows_affected().eq(&0) {
                            transaction.rollback().await?;
                            return Err(VaderError::TeamNotFound(
                                "No Team found to Add Team Members",
                            ));
                        }
                    }
                    Err(err) => {
                        transaction.rollback().await?;
                        log::error!("Unable to add member :  {}", mem_id);
                        return Err(VaderError::SqlxError(err));
                    }
                }
            }
            transaction.commit().await?;
            Ok(())
        })
    }
}
impl<'a> VaderEvent<'a> for Event<'a, Team<'a>> {
    type Participant = Team<'a>;
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
            if let Some(team_size) = self.team_size {
                sqlx::query!(
                    "INSERT INTO events (id,name,logo,event_type,team_size) VALUES (?,?,?,?,?)",
                    id,
                    name,
                    logo,
                    "team_event",
                    team_size,
                )
                .execute(db_pool)
                .await?;
                Ok(())
            } else {
                return Err(VaderError::TeamSizeMismatch("Team size not specified"));
            }
        })
    }
    fn get_logo(&self) -> String {
        self.logo.clone().unwrap_or(Cow::Borrowed("")).to_string()
    }
}

impl<'a> VaderEvent<'a> for Event<'a, User<'a>> {
    type Participant = User<'a>;
    fn add_participant(
        &'a self,
        participant: &Self::Participant,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, ()> {
        Self::add_participant_from_id(self, participant.get_id(), db_pool)
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
                name,
                logo,
                "user_event"
            )
            .execute(db_pool)
            .await?;
            Ok(())
        })
    }
    fn get_logo(&self) -> String {
        self.logo.clone().unwrap_or(Cow::Borrowed("")).to_string()
    }
}

impl<'a> Event<'a, User<'a>, ActiveEvent> {
    pub fn update_score_by_id(
        &self,
        user_id: &Uuid,
        points: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, i32> {
        let id = user_id.to_string();
        Box::pin(async move {
            let res = sqlx::query_as::<_, (i32,)>(
                "UPDATE users set score=score+? WHERE id=? RETURNING score",
            )
            .bind(points)
            .bind(id)
            .fetch_one(db_pool)
            .await;
            match res {
                Ok(score) => Ok(score.0),
                Err(err) => return Err(VaderError::SqlxError(err)),
            }
        })
    }
}

impl<'a> Event<'a, Team<'a>> {
    pub fn add_team_members(
        &'a self,
        team_id: &'a Uuid,
        members: &'a [Uuid],
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<()> {
        Team::add_members_from_id(team_id, members, db_pool)
    }
}

impl<'a> Event<'a, Team<'a>, ActiveEvent> {
    pub fn update_score_by_id(
        &self,
        team_id: &Uuid,
        points: i64,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, i32> {
        let id = team_id.to_string();
        Box::pin(async move {
            let res = sqlx::query_as::<_, (i32,)>(
                "UPDATE teams set score=score+? WHERE id=? RETURNING score",
            )
            .bind(points)
            .bind(id)
            .fetch_one(db_pool)
            .await;
            match res {
                Ok(score) => Ok(score.0),
                Err(err) => return Err(VaderError::SqlxError(err)),
            }
        })
    }
}

impl<'a> Event<'a, Team<'a>, NewEvent> {
    pub fn reset_score(&self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
        let event_id = self.id.to_string();
        Box::pin(async move {
            sqlx::query!("UPDATE teams SET score=0 WHERE id IN(SELECT t.id FROM events e JOIN event_teams et ON et.event_id = e.id JOIN teams t ON et.team_id = t.id WHERE e.id = ?)",event_id).execute(db_pool).await?;
            sqlx::query!("UPDATE users SET score=0 WHERE id IN(SELECT u.id FROM events e JOIN event_teams et ON et.event_id = e.id JOIN teams t ON et.team_id = t.id JOIN team_members tm ON tm.team_id = t.id JOIN users u ON tm.user_id = u.id WHERE e.id = ? )",event_id).execute(db_pool).await?;
            Ok(())
        })
    }
}
impl<'a> Event<'a, User<'a>, NewEvent> {
    pub fn reset_score(&self, db_pool: &'a SqlitePool) -> AsyncDbRes<'a, ()> {
        let event_id = self.id.to_string();
        Box::pin(async move {
            sqlx::query!("UPDATE users SET score=0 WHERE id IN(SELECT u.id FROM events e JOIN event_users eu ON eu.event_id = e.id JOIN users u ON eu.user_id = u.id WHERE e.id = ?)",event_id).execute(db_pool).await?;
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

impl<'a, T> From<&Event<'a, T, NewEvent>> for Event<'a, T, ActiveEvent>
where
    T: Player<'a>,
{
    fn from(e: &Event<'a, T, NewEvent>) -> Self {
        Event {
            id: e.id,
            name: e.name.clone(),
            logo: e.logo.clone(),
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a ActiveEvent>,
            team_size: e.team_size,
        }
    }
}
impl<'a, T> From<&Event<'a, T, ActiveEvent>> for Event<'a, T, EndEvent>
where
    T: Player<'a>,
{
    fn from(e: &Event<'a, T, ActiveEvent>) -> Self {
        Event {
            id: e.id,
            name: e.name.clone(),
            logo: e.logo.clone(),
            player_marker: PhantomData::<&'a T>,
            state_marker: PhantomData::<&'a EndEvent>,
            team_size: e.team_size,
        }
    }
}

impl<'a> Team<'a> {
    pub fn new(name: Cow<'a, str>, logo: Option<Cow<'a, str>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            members: Vec::new(),
            score: 0,
        }
    }
    pub fn get_team<'b>(team_id: &'b Uuid, db_pool: &'b SqlitePool) -> AsyncDbRes<'b, Self> {
        let id = team_id.to_string();
        Box::pin(async move {
            let team = sqlx::query_as::<_, Team>(
                "SELECT t.id AS id,t.name AS name, t.score AS score,t.logo AS logo,GROUP_CONCAT(tm.user_id,',') AS team_members FROM teams t JOIN team_members tm ON tm.team_id = t.id WHERE t.id = ? GROUP BY t.id",
            )
            .bind(id)
            .fetch_one(db_pool)
            .await?;
            Ok(team)
        })
    }
    pub fn delete_team<'b>(id: &'b Uuid, db_pool: &'b SqlitePool) -> AsyncDbRes<'b, ()> {
        let id = id.to_string();
        Box::pin(async move {
            let res = sqlx::query!("DELETE FROM teams WHERE id = ? ", id)
                .execute(db_pool)
                .await?;
            if res.rows_affected().eq(&0) {
                return Err(VaderError::TeamNotFound("No team found"));
            }
            Ok(())
        })
    }
}
impl<'a> User<'a> {
    pub fn new(name: Cow<'a, str>, logo: Option<Cow<'a, str>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            logo,
            score: 0,
        }
    }
    pub fn get_user<'b>(user_id: &'b Uuid, db_pool: &'b SqlitePool) -> AsyncDbRes<'b, Self> {
        let id = user_id.to_string();
        Box::pin(async move {
            let user =
                sqlx::query_as::<_, User>("SELECT id,name,score,logo FROM users WHERE id = ?")
                    .bind(id)
                    .fetch_one(db_pool)
                    .await?;
            Ok(user)
        })
    }
    pub fn delete_user<'b>(id: &'b Uuid, db_pool: &'b SqlitePool) -> AsyncDbRes<'b, ()> {
        let id = id.to_string();
        Box::pin(async move {
            let res = sqlx::query!("DELETE FROM users WHERE id = ? ", id)
                .execute(db_pool)
                .await?;
            if res.rows_affected().eq(&0) {
                return Err(VaderError::UserNotFound("No User found"));
            }
            Ok(())
        })
    }
    pub fn get_all_user(db_pool: &SqlitePool) -> AsyncDbRes<'_, Vec<Self>> {
        Box::pin(async move {
            let users = sqlx::query_as::<_, User>("SELECT id,name,score,logo from users")
                .fetch_all(db_pool)
                .await?;
            Ok(users)
        })
    }
    pub fn event_user_fts(
        event_id: &Uuid,
        param: &'a str,
        db_pool: &'a SqlitePool,
    ) -> AsyncDbRes<'a, Vec<Self>> {
        let event_id = event_id.to_string();
        Box::pin(async move {
            let users = sqlx::query_as::<_, User>(
            "SELECT id,name,score,logo FROM users_fts u JOIN event_users ut ON ut.user_id=u.id WHERE ut.event_id = ? AND name MATCH  ? ",
            )
            .bind(&event_id)
            .bind(format!("{}*", param))
            .fetch_all(db_pool)
            .await?;
            Ok(users)
        })
    }
}

pub trait VbStateMarker {}
impl VbStateMarker for ActiveEvent {}
impl VbStateMarker for EndEvent {}

pub trait VaderBoard<'a> {
    type VbRes: Queriable;
    fn get_vboard<'b>(
        &'a self,
        db_pool: &'b SqlitePool,
        count: u32,
    ) -> AsyncDbRes<'a, Vec<Self::VbRes>>
    where
        'b: 'a;
}

impl<'a, S: VbStateMarker + EventState> VaderBoard<'a> for Event<'a, Team<'a>, S> {
    type VbRes = TeamInfo<'a>;
    fn get_vboard<'b>(
        &'a self,
        db_pool: &'b SqlitePool,
        count: u32,
    ) -> AsyncDbRes<'a, Vec<Self::VbRes>>
    where
        'b: 'a,
    {
        Box::pin(async move {
            let teams = sqlx::query_as::<_, TeamInfo>(
                "SELECT id,name,score,logo FROM teams ORDER BY score DESC LIMIT ?",
            )
            .bind(count)
            .fetch_all(db_pool)
            .await?;
            Ok(teams)
        })
    }
}

impl<'a, S: VbStateMarker + EventState> VaderBoard<'a> for Event<'a, User<'a>, S> {
    type VbRes = User<'a>;
    fn get_vboard<'b>(
        &'a self,
        db_pool: &'b SqlitePool,
        count: u32,
    ) -> AsyncDbRes<'a, Vec<Self::VbRes>>
    where
        'b: 'a,
    {
        Box::pin(async move {
            let users = sqlx::query_as::<_, User>(
                "SELECT id,name,score,logo FROM users ORDER BY score DESC LIMIT ?",
            )
            .bind(count)
            .fetch_all(db_pool)
            .await?;
            Ok(users)
        })
    }
}
