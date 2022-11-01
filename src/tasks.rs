use std::time::{SystemTime, Duration};

use bson::{Document, Uuid, doc};
use log::{info, warn};
use mongodb::{options::{FindOptions, FindOneOptions}, Collection};
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::{
    db::{errors::DbError, DbQuery, SocialsDb, DbActions},
    social::{
        dzen_core::DzenCore, ok_core::OkCore, source::SocialSource, vk_core::VkCore,
        yt_core::YtCore, SocialCore, SocialPlatform, post::SocialPost,
    }, bots::{BotLimitSleep, Bot}, utils::{mdb_cond_or_null, unix_now_secs_f64},
};

use self::{errors::TaskError, like::LikeAction, watch::WatchAction};

// local modules
pub mod errors;
pub mod like;
pub mod tests;
pub mod watch;

pub mod events;
// eof local modules

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskTarget { Dummy, Video, Post, User, Group }

impl Default for TaskTarget {
    fn default() -> Self { Self::Dummy }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskActionType { Like, Watch, Dummy }

impl Default for TaskActionType {
    fn default() -> Self {
        Self::Dummy
    }
}

impl TaskActionType {
    fn is_default_browser(&self, p: &SocialPlatform) -> bool {
        match self {
            Self::Watch => true,
            Self::Like => match p {
                SocialPlatform::Vk => false,
                _ => true
            },
            _ => false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BotTaskStatus { Active, Processing, Stopped, Error, Finished }

impl Default for BotTaskStatus {
    fn default() -> Self {
        return Self::Active;
    }
}

// TODO improve
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BotTaskQuery {
    pub id: Option<bson::Uuid>,
    pub source_id: Option<bson::Uuid>,
    pub title: Option<String>,
    pub platform: Option<SocialPlatform>,
    pub status: Option<BotTaskStatus>,
    pub is_active: Option<u8>,
    pub is_locked: Option<bool>,
    pub include_hidden: Option<u8>,
    pub is_browser: Option<u8>,
    pub include_browser_tasks: Option<u8>,
    pub sort_by_created_date: Option<i32>,
    pub sort_by_updated_date: Option<i32>,
    pub skip: Option<u64>,
    pub limit: Option<i64>,
    pub not_sleep: Option<bool>
}

impl BotTaskQuery {
    pub fn new() -> Self { Self::default() }
    pub fn is_active(&mut self) -> &mut Self { self.status = Some(BotTaskStatus::Active); self }
    pub fn not_action_locked(&mut self) -> &mut Self { self.is_locked = Some(false); self }
    pub fn is_finished(&mut self) -> &mut Self { self.status = Some(BotTaskStatus::Finished); self }
    pub fn is_browser(&mut self) -> &mut Self { self.is_browser = Some(1); self }
    pub fn not_browser(&mut self) -> &mut Self { self.is_browser = Some(0); self }
    pub fn top_old_updated(&mut self) -> &mut Self { self.sort_by_updated_date = Some(1); self }
    pub fn top_fresh_updated(&mut self) -> &mut Self { self.sort_by_updated_date = Some(-1); self }
    pub fn not_sleep(&mut self) -> &mut Self { self.not_sleep = Some(true); self }
}

impl DbQuery for BotTaskQuery {
    fn collect_filters(&self) -> Document {
        let mut f = Document::new();
        if let Some(i) = &self.id { f.insert("id", i); }
        if let Some(i) = &self.source_id { f.insert("social_source.id", i); }
        if let Some(i) = &self.title { f.insert("title", i.as_str()); }
        if let Some(p) = &self.platform { f.insert("platform", to_value(p).unwrap().as_str()); }
        if let Some(i) = &self.status { f.insert("status", to_value(i).unwrap().as_str()); }
        if let Some(i) = &self.is_active { f.insert("is_active", *i != 0); }

        if let Some(i) = &self.is_browser {
            let b = match i { 0 => false, 1 => true, _ => false, };
            f.insert("options.is_browser", b);
        }

        if let Some(_i) = &self.not_sleep {
            let key: &str = "next_run_time.secs_since_epoch";
            mdb_cond_or_null(
                &mut f, key,
                doc! { "$lte": unix_now_secs_f64() }
            );
        }
        f
    }

    fn collect_sorting(&self) -> Document {
        let mut s = Document::new();
        if let Some(i) = &self.sort_by_created_date { s.insert("date_created.secs_since_epoch", i); }
        if let Some(i) = &self.sort_by_created_date { s.insert("date_created.nanos_since_epoch", i); }
        if let Some(i) = &self.sort_by_updated_date { s.insert("date_updated.secs_since_epoch", i); }
        if let Some(i) = &self.sort_by_updated_date { s.insert("date_updated.nanos_since_epoch", i); }
        s
    }

    fn collect_options(&self) -> FindOptions {
        let mut f = FindOptions::default();
        f.skip = self.skip;
        f.limit = self.limit;
        f.sort = Some(self.collect_sorting());
        f
    }

    fn collect_one_options(&self) -> FindOneOptions {
        let mut f = FindOneOptions::default();
        f.sort = Some(self.collect_sorting());
        f
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BotTaskCreate {
    pub is_active: bool,
    pub title: String,
    pub platform: SocialPlatform,
    pub is_testing: bool,
    pub action_type: TaskActionType,
    pub action: TaskActionEnum,
    #[serde(default)]
    pub social_source_id: Option<bson::Uuid>,
    #[serde(default)]
    pub extra: TaskExtra,
}

impl BotTaskCreate {
    pub fn new(task: Self) -> Self {
        Self { ..task }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone )]
pub struct BotTaskOptions {
    pub delete_after_finished: bool,
    pub is_hidden: bool,
    pub is_testing: bool,
    pub is_browser: bool,
}

impl Default for BotTaskOptions {
    fn default() -> Self {
        Self {
            delete_after_finished: false,
            is_hidden: false,
            is_testing: false,
            is_browser: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskActionEnum {
    LikeAction(LikeAction),
    WatchAction(WatchAction),
    Test,
}

impl Default for TaskActionEnum {
    fn default() -> Self {
        Self::Test
    }
}

impl TaskActionEnum {
    pub fn target (&self) -> TaskTarget {
        match self {
            Self::LikeAction(a) => a.target(),
            Self::WatchAction(a) => a.target(),
            _ => TaskTarget::Dummy,
        }
    }

    fn need_run(task: &mut BotTask) -> bool {
        let action = task.action.clone();
        match action {
            Self::LikeAction(a) => a.need_run(task),
            Self::WatchAction(a) => a.need_run(task),
            _ => false,
        }
    }
    fn check_done(task: &mut BotTask) -> bool {
        let action = task.action.clone();
        match action {
            Self::LikeAction(a) => a.check_done(task),
            Self::WatchAction(a) => a.check_done(task),
            _ => false,
        }
    }
    fn calc_next_time_run(task: &mut BotTask) {
        let action = task.action.clone();
        match action {
            Self::LikeAction(a) => a.calc_next_time_run(task),
            Self::WatchAction(a) => a.calc_next_time_run(task),
            _ => (),
        }
    }

    pub fn use_browser(&self) -> bool {
        match self {
            Self::LikeAction(a) => a.use_browser(),
            Self::WatchAction(a) => a.use_browser(),
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskExtra {
    pub post: Option<SocialPost>
}

impl TaskExtra {
    pub fn with_post(&mut self, v: SocialPost) -> &mut Self { self.post = Some(v); self }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotTask {
    pub id: bson::Uuid,
    pub is_active: bool,
    // is_locked: bool,
    status: BotTaskStatus,
    pub date_created: SystemTime,
    pub date_updated: SystemTime,
    pub title: String,
    pub platform: SocialPlatform,
    pub next_run_time: Option<SystemTime>,
    pub options: BotTaskOptions,
    error: Option<TaskError>,
    pub action_type: TaskActionType,
    pub action: TaskActionEnum,
    #[serde(default="TaskExtra::default")]
    pub extra: TaskExtra,
    pub social_source: Option<SocialSource>,
}

impl BotTask {
    pub fn print_info(&self) { println!("{} {:#?}", self.title, self.platform) }
    pub fn deactivate(&mut self) -> &mut Self { self.is_active = false; self }

    pub fn set_status_active(&mut self) -> &mut Self { self.status = BotTaskStatus::Active; self }
    pub fn set_status_processing(&mut self) -> &mut Self { self.status = BotTaskStatus::Processing; self }
    pub fn set_status_stopped(&mut self) -> &mut Self { self.status = BotTaskStatus::Stopped; self }
    pub fn set_status_error(&mut self) -> &mut Self { self.status = BotTaskStatus::Error; self }
    pub fn set_status_finished(&mut self) -> &mut Self { self.status = BotTaskStatus::Finished; self }

    pub fn sleep_no_bots(&mut self, sleep: Option<Duration>) -> &mut Self {
        let now = SystemTime::now();
        let sleep = sleep.unwrap_or(Duration::from_secs(300));
        let wait = now.checked_add(sleep).unwrap();
        info!("Task fall sleep for {} seconds", sleep.as_secs());
        self.next_run_time = Some(wait); self
    }

    pub fn is_done(&self) -> bool {
        match self.status {
            BotTaskStatus::Finished => true,
            _ => false,
        }
    }

    pub fn set_done(&mut self) -> &mut Self {
        info!("Task is done!");
        self.status = BotTaskStatus::Finished;
        self.is_active = false;
        self
    }

    pub fn process_error(&mut self, e: TaskError) -> &mut Self {
        // TODO special cases? 🤔
        info!("[BotTask] {} processing error {:#?}", self.id, e);
        self.set_error(e).set_status_error()
    }
    pub fn set_error(&mut self, e: TaskError) -> &mut Self {
        self.error = Some(e); self.is_active = false; self
    }
    pub fn clear_error(&mut self) -> &mut Self { self.error = None; self }

    /*
    pub async fn lock_db(
    &mut self,
    db: &SocialsDb
    ) -> Result<mongodb::results::UpdateResult, DbError> {
    self.is_locked = true;
    self.update_db(db).await
    }

    pub async fn unlock_db(
    &mut self,
    db: &SocialsDb
    ) -> Result<mongodb::results::UpdateResult, DbError> {
    self.is_locked = false;
    self.update_db(db).await
    }
    */

    pub async fn get_fresh(
        &mut self,
        db: &SocialsDb
    ) -> Result<&mut Self, DbError> {
        let q = BotTaskQuery {
            id: Some(self.id),
            ..Default::default()
        };
        match SocialsDb::find_one(&q, &db.bots_tasks())
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
        // update task date_updated
        self.date_updated = SystemTime::now();
        SocialsDb::update_by_id(self.id, self, &db.bots_tasks()).await
    }

    async fn make_v2(&mut self, db: &SocialsDb) {
        info!("--- INVOKE MAKE TASK {} ---", self.id);
        self.check_calc_next_time_run();
        let need_run = self.need_run();
        info!("Need run task: {}", need_run);
        if need_run == false {
            info!("Task dont need to be run");
            return;
        }

        let vk_core = VkCore::new();
        let ok_core = OkCore::new();
        let yt_core = YtCore::new();
        let dzen_core = DzenCore::new();

        match self.platform {
            SocialPlatform::Vk => vk_core.make_action(self, db).await,
            SocialPlatform::Ok => ok_core.make_action(self, db).await,
            SocialPlatform::Youtube => yt_core.make_action(self, db).await,
            SocialPlatform::Dzen => dzen_core.make_action(self, db).await,
            _ => info!("{:#?} not implemented yet", self.platform),
        }
    }

    pub async fn make(&mut self, db: &SocialsDb) { self.make_v2(db).await; }
    pub fn check_done(&mut self) -> bool { TaskActionEnum::check_done(self) }
    pub fn need_run(&mut self) -> bool { TaskActionEnum::need_run(self) }

    pub fn check_calc_next_time_run(&mut self) {
        match self.next_run_time {
            Some(_) => (),
            None => self.calc_next_time_run(),
        }
    }
    pub fn calc_next_time_run(&mut self) { TaskActionEnum::calc_next_time_run(self) }
    pub fn has_error(&self) -> bool { match self.error { Some(_) => true, None => false, } }

    // TODO convert into result?
    pub async fn create_from(db: &SocialsDb, t: BotTaskCreate) -> BotTask {
        let social_source = match t.social_source_id {
            None => None,
            Some(id) => SocialSource::find_by_id(id, db.social_sources())
                .await
                .unwrap(),
        };
        // TODO!
        let options = BotTaskOptions {
            is_testing: t.is_testing,
            is_browser: t.action_type.is_default_browser(&t.platform),
            ..Default::default()
        };
        BotTask {
            id: Uuid::new(),
            is_active: t.is_active,
            // is_locked: false,
            status: BotTaskStatus::default(),
            date_created: SystemTime::now(),
            date_updated: SystemTime::now(),
            title: t.title,
            platform: t.platform,
            next_run_time: None,
            error: None,
            action_type: t.action_type,
            action: t.action,
            options,
            social_source,
            extra: t.extra
        }
    }
}

impl DbActions for BotTask {
    type Query = BotTaskQuery;
    fn get_collection(&self,db: &SocialsDb) -> Collection<Self> { db.bots_tasks() }
    fn get_id(&self) -> bson::Uuid { self.id }
}

pub trait TaskAction {

    fn action_type(&self) -> TaskActionType;
    fn bot_assign_sleep(&self, _bot: &mut Bot, _sleep: Duration) {
        warn!("Call bot sleep from TaskAction. Not implemented");
    }
    fn bot_min_sleep(&self) -> Duration { Duration::from_secs(60) }
    fn bot_1hr_limit_sleep(&self) -> BotLimitSleep {
        BotLimitSleep { limit: 3, sleep: Duration::from_secs(300) }
    }
    fn bot_24hr_limit_sleep(&self) -> BotLimitSleep {
        BotLimitSleep { limit: 12, sleep: Duration::from_secs(18000) }
    }
    fn target(&self) -> TaskTarget;
    fn do_stuff(&self) { println!("some stuff there") }
    fn need_run(&self, task: &mut BotTask) -> bool {
        if task.has_error() { return false; }
        let time_now = SystemTime::now();
        let next_run_time = task.next_run_time;
        match next_run_time {
            Some(t) => time_now >= t,
            None => false,
        }
    }
    fn check_done(&self, task: &mut BotTask) -> bool;
    fn calc_need_do_now(&self, task: &BotTask) -> u64;
    fn calc_next_time_run(&self, task: &mut BotTask);
    fn use_browser(&self) -> bool { false }
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BotTaskTypeTarget {
    pub target: TaskTarget,
    #[serde(default="Vec::new")]
    pub platforms: Vec<SocialPlatform>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotTaskType {
    #[serde(default)]
    pub id: bson::Uuid,
    pub action_type: TaskActionType,
    name: String,
    #[serde(default="String::new")]
    description: String,
    #[serde(default="Vec::new")]
    pub targets: Vec<BotTaskTypeTarget>,
    pub is_active: bool
}

impl Default for BotTaskType {
    fn default () -> Self {
        Self {
            id: bson::Uuid::new(),
            action_type: TaskActionType::Dummy,
            name: "Testing".to_string(),
            description: "testing".to_string(),
            targets: Vec::new(),
            is_active: false
        }
    }
}
