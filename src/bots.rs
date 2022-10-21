use std::time::SystemTime;

use serde::{Serialize, Deserialize};

use crate::{social::SocialPlatform, db::{SocialsDb, errors::DbError}};

use self::{query::BotQuery, errors::BotError};

#[cfg(test)]
pub mod tests;

// local moduels
pub mod query;
pub mod errors;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BotStatus { Configure, Ready, Resting, InUse, Banned, ActionRequired, Error }

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct BotPlatformData {
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotExtra { pub notes: Option<String> }
impl BotExtra {
    fn init () -> Self { Self {..Default::default()} }
}

impl BotPlatformData {
    fn init () -> Self { Self {..Default::default()} }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default )]
pub struct BotCreate {
    pub social_id: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub platform: SocialPlatform,
    pub created_source: Option<String>,
    pub make_ready: bool
}

#[derive(Debug, Serialize, Deserialize, Clone )]
pub struct Bot {
    pub id: bson::Uuid,
    pub social_id: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub access_token: Option<String>,
    // times
    pub date_created: SystemTime,
    pub date_updated: SystemTime,
    pub last_used: Option<SystemTime>,
    pub rest_until: Option<SystemTime>,
    // eof times
    pub platform: SocialPlatform,
    pub status: BotStatus,
    created_source: Option<String>,
    platform_data: BotPlatformData,
    pub extra: BotExtra,
    pub error: Option<BotError>
}

impl Bot {
    // status helpers
    pub fn is_ready (&self) -> bool { match self.status { BotStatus::Ready => true, _ => false }}
    pub fn is_resting (&self) -> bool { match self.status { BotStatus::Ready => true, _ => false }}
    pub fn is_banned(&self) -> bool { match self.status { BotStatus::Banned => true, _ => false }}
    pub fn is_need_action(&self) -> bool { match self.status { BotStatus::ActionRequired => true, _ => false }}
    pub fn is_in_use(&self) -> bool { match self.status { BotStatus::InUse => true, _ => false }}
    pub fn is_error(&self) -> bool { match self.status { BotStatus::Error => true, _ => false }}

    pub fn set_status_error(&mut self) -> &mut Self { self.status = BotStatus::Error; self }
    pub fn set_status_banned(&mut self) -> &mut Self { self.status = BotStatus::Error; self }
    pub fn set_status_ready(&mut self) -> &mut Self { self.status = BotStatus::Error; self }
    pub fn set_status_in_use(&mut self) -> &mut Self { self.status = BotStatus::Error; self }
    pub fn set_status_action_required(&mut self) -> &mut Self { self.status = BotStatus::Error; self }
    // eof status helpers
    // db helpers
    pub async fn get_fresh(
        &mut self,
        db: &SocialsDb
    ) -> Result<&mut Self, DbError> {
        let q = BotQuery {
            id: Some(self.id),
            ..Default::default()
        };
        match SocialsDb::find_one(&q, &db.bots())
            .await {
                Ok(r) => match r {
                    Some(t) => { *self = t; Ok(self) }
                    _ => Ok(self)
                },
                Err(e) => Err(e)
            }
    }

    pub async fn update_db(
        &mut self,
        db: &SocialsDb,
    ) -> Result<mongodb::results::UpdateResult, DbError> {
        self.date_updated = SystemTime::now();
        SocialsDb::update_by_id(self.id, self.clone(), &db.bots()).await
    }
    // eof db helpers
    // error helpers
     pub fn process_error(&mut self, e: BotError) -> &mut Self {
        // TODO special cases? 🤔
        self.set_error(e).set_status_error()
    }
    pub fn set_error(&mut self, e: BotError) -> &mut Self {
        self.error = Some(e); self
    }
    pub fn clear_error(&mut self) -> &mut Self { self.error = None; self }

    pub async fn create_from(_db: &SocialsDb, v: BotCreate) -> Result<Bot, String> {
        let status = match v.make_ready {
            true => BotStatus::Ready,
            false => BotStatus::Configure
        };
        let bot = Bot {
            id: bson::Uuid::new(),
            social_id: v.social_id,
            username: v.username,
            password: v.password,
            access_token: v.access_token,
            date_created: SystemTime::now(),
            date_updated: SystemTime::now(),
            last_used: None,
            rest_until: None,
            platform: v.platform,
            status,
            created_source: None,
            platform_data: BotPlatformData::init(),
            extra: BotExtra::init(),
            error: None
        };
        Ok(bot)
    }
}
