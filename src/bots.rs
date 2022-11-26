use std::time::{Duration, SystemTime};

use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    db::{errors::DbError, DbActions, SocialsDb},
    social::{vk_core::VkCore, SocialPlatform},
    tasks::{events::ActionEvent, TaskAction},
    utils::pretty_duration,
};

use self::{actions::SocialBotActions, errors::BotError, query::BotQuery};

#[cfg(test)]
pub mod tests;

// local moduels
pub mod actions;
pub mod errors;
pub mod query;

#[derive(Debug, Clone, Default)]
pub struct BotLimitSleep {
    pub limit: u64,
    pub sleep: Duration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BotStatus {
    Configure,
    Ready,
    Resting,
    InUse,
    Banned,
    ActionRequired,
    Error,
}
impl Default for BotStatus {
    fn default() -> Self {
        Self::Configure
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotPlatformData {
    pub refresh_token: Option<String>,
    pub expires_in: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotExtra {
    pub notes: Option<String>,
}
impl BotExtra {
    fn init() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl BotPlatformData {
    fn init() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotActionsRest {
    pub like: Option<SystemTime>,
    pub repost: Option<SystemTime>,
    pub comment: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotCreate {
    pub social_id: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub platform: SocialPlatform,
    pub created_source: Option<String>,
    pub make_ready: bool,
    pub gender: Option<Gender>,
    pub rest_until: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotUpdate {
    pub social_id: Option<String>,
    pub username: String,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub platform: SocialPlatform,
    pub status: BotStatus,
    pub gender: Option<Gender>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(default = "BotActionsRest::default")]
    pub actions_rest: BotActionsRest,
    // eof times
    pub platform: SocialPlatform,
    pub status: BotStatus,
    pub created_source: Option<String>,
    pub platform_data: BotPlatformData,
    pub extra: BotExtra,
    pub error: Option<BotError>,
    pub gender: Option<Gender>,
}

impl Bot {
    // status helpers
    pub fn is_ready(&self) -> bool { match self.status { BotStatus::Ready => true, _ => false, } }
    pub fn is_resting(&self) -> bool { match self.status { BotStatus::Resting => true, _ => false, } }
    pub fn is_banned(&self) -> bool { match self.status { BotStatus::Banned => true, _ => false, } }
    pub fn is_need_action(&self) -> bool { match self.status { BotStatus::ActionRequired => true, _ => false, } }
    pub fn is_in_use(&self) -> bool { match self.status { BotStatus::InUse => true, _ => false, } }
    pub fn is_error(&self) -> bool { match self.status { BotStatus::Error => true, _ => false, } }

    pub fn set_status_error(&mut self) -> &mut Self { self.status = BotStatus::Error; self }
    pub fn set_status_banned(&mut self) -> &mut Self { self.status = BotStatus::Banned; self }
    pub fn set_status_ready(&mut self) -> &mut Self { self.status = BotStatus::Ready; self }
    pub fn set_status_resting(&mut self) -> &mut Self { self.status = BotStatus::Resting; self }
    pub fn set_status_in_use(&mut self) -> &mut Self { self.status = BotStatus::InUse; self }
    pub fn set_status_action_required(&mut self) -> &mut Self { self.status = BotStatus::ActionRequired; self }
    // eof status helpers
    //
    // sleep helpers
    pub async fn after_action_sleep(
        &mut self,
        action: &impl TaskAction,
        db: &SocialsDb,
    ) -> &mut Self {
        // TODO fix items len to total
        // 24hr limit check
        let l24 = action.bot_24hr_limit_sleep();
        let last_24hr: u64 =
            ActionEvent::get_bot_last_24hr_events(&self.id, db, Some(action.action_type()))
                .await
                .unwrap()
                .items
                .len()
                .try_into()
                .unwrap();
        if last_24hr >= l24.limit {
            info!(
                "[Bot 24hr limit sleep] {} reach 24hr limit: {}",
                self.id, l24.limit
            );
            action.bot_assign_sleep(self, l24.sleep);
            return self;
        }

        // 1hr limit check
        let last_1hr: u64 =
            ActionEvent::get_bot_last_1hr_events(&self.id, db, Some(action.action_type()))
                .await
                .unwrap()
                .items
                .len()
                .try_into()
                .unwrap();
        let l1 = action.bot_1hr_limit_sleep();
        if last_1hr >= l1.limit {
            info!(
                "[Bot 1hr limit sleep] {} reach 1hr limit: {}",
                self.id, l1.limit
            );
            action.bot_assign_sleep(self, l1.sleep);
            return self;
        }

        // regular task sleep delay
        let regular_sleep = action.bot_min_sleep();
        info!(
            "[Bot regular sleep] Bot: {} . Sleep for {}. metrics: |{} in 1hr| |{} in 24hr|",
            self.id,
            pretty_duration(regular_sleep),
            last_1hr,
            last_24hr
        );
        action.bot_assign_sleep(self, regular_sleep);
        return self;
    }
    // eof sleep helpers

    // db helpers
    pub async fn get_fresh(&mut self, db: &SocialsDb) -> Result<&mut Self, DbError> {
        let q = BotQuery {
            id: Some(self.id),
            ..Default::default()
        };
        match SocialsDb::find_one(&q, &db.bots()).await {
            Ok(r) => match r {
                Some(t) => {
                    *self = t;
                    Ok(self)
                }
                _ => Ok(self),
            },
            Err(e) => Err(e),
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
        info!("[Bot] {} processing error {:#?}", self.id, e);
        self.set_error(e).set_status_error()
    }
    pub fn set_error(&mut self, e: BotError) -> &mut Self {
        self.error = Some(e);
        self
    }
    pub fn clear_error(&mut self) -> &mut Self {
        self.error = None;
        self
    }

    pub fn update_with(&mut self, b: BotUpdate) -> &mut Self {
        self.social_id = b.social_id;
        self.username = b.username;
        self.password = b.password;
        self.access_token = b.access_token;
        self.platform = b.platform;
        self.status = b.status;
        self.default_checks();
        self
    }

    pub fn check_global_sleep(&mut self) -> &mut Self {
        match &self.rest_until {
            None => if self.is_resting() { self.set_status_ready() } else { self },
            Some(v) => match SystemTime::now().ge(v) {
                true => { self.rest_until = None; self.set_status_error() },
                false => { self.set_status_resting() }
            }
        }
    }

    pub fn default_checks(&mut self) -> &mut Self {
        if self.is_error() || self.is_banned() || self.is_need_action() || self.is_in_use() { return self }
        self.check_global_sleep()
    }

    pub async fn create_from(_db: &SocialsDb, v: BotCreate) -> Result<Bot, String> {
        let status = match v.make_ready {
            true => BotStatus::Ready,
            false => BotStatus::Configure,
        };
        let mut bot = Bot {
            id: bson::Uuid::new(),
            social_id: v.social_id,
            username: v.username,
            password: v.password,
            access_token: v.access_token,
            date_created: SystemTime::now(),
            date_updated: SystemTime::now(),
            last_used: None,
            rest_until: v.rest_until,
            actions_rest: BotActionsRest::default(),
            platform: v.platform,
            status,
            created_source: None,
            platform_data: BotPlatformData::init(),
            extra: BotExtra::init(),
            error: None,
            gender: v.gender,
        };
        bot.default_checks();
        Ok(bot)
    }

    pub async fn fetch_by_access_token(
        platform: SocialPlatform,
        token: &str,
    ) -> Result<Bot, BotError> {
        match platform {
            SocialPlatform::Vk => VkCore::fetch_by_access_token(token).await,
            _ => Err(BotError::not_implemented(
                Some(&format!( "[NotImplemented] fetch_by_access_token for {:#?}", platform)),
                None,
            )),
        }
    }
}

impl DbActions for Bot {
    type Query = BotQuery;
    fn get_collection(&self, db: &SocialsDb) -> mongodb::Collection<Self> {
        db.bots()
    }
    fn get_id(&self) -> bson::Uuid {
        self.id
    }
}
