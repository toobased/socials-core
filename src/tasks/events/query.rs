use std::time::{SystemTime, UNIX_EPOCH, Duration};

use bson::{Document, doc};
use serde::{Serialize, Deserialize};
use serde_json::to_value;

use crate::{db::DbQuery, tasks::{TaskActionType, TaskTarget}, social::SocialPlatform};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ActionEventQuery {
    pub bot_id: Option<bson::Uuid>,
    pub task_id: Option<bson::Uuid>,
    pub action_type: Option<TaskActionType>,
    pub task_target: Option<TaskTarget>,
    pub platform: Option<SocialPlatform>,
    pub date_created_gte: Option<SystemTime>,
    pub date_created_lte: Option<SystemTime>
}

impl ActionEventQuery {
    pub fn with_bot_id(&mut self, v: bson::Uuid) -> &mut Self { self.bot_id = Some(v); self }
    pub fn with_task_id(&mut self, v: bson::Uuid) -> &mut Self { self.task_id = Some(v); self }
    pub fn with_action_type(&mut self, v: TaskActionType) -> &mut Self { self.action_type = Some(v); self }
    pub fn with_task_target(&mut self, v: TaskTarget) -> &mut Self { self.task_target = Some(v); self }
    pub fn with_platform(&mut self, v: SocialPlatform) -> &mut Self { self.platform = Some(v); self }
    pub fn with_date_created_gte(&mut self, v: SystemTime) -> &mut Self { self.date_created_gte = Some(v); self }
    pub fn with_date_created_lte(&mut self, v: SystemTime) -> &mut Self { self.date_created_lte = Some(v); self }
    pub fn with_last_hrs(&mut self, h: u64) -> &mut Self {
        let secs = h.checked_mul(120).unwrap_or(0);
        let v = SystemTime::now().checked_sub(Duration::from_secs(secs)).unwrap();
        self.date_created_gte = Some(v); self
    }
    pub fn with_last_1hr(&mut self) -> &mut Self { self.with_last_hrs(1) }
    pub fn with_last_24hr(&mut self) -> &mut Self { self.with_last_hrs(24) }
}

impl DbQuery for ActionEventQuery {
    fn collect_filters(&self) -> bson::Document {
        let mut f = Document::new();
        if let Some(v) = &self.bot_id { f.insert("bot_id", v); }
        if let Some(v) = &self.task_id { f.insert("task_id", v); }
        if let Some(v) = &self.action_type { f.insert("action_type", to_value(v).unwrap().as_str()); }
        if let Some(v) = &self.task_target { f.insert("task_target", to_value(v).unwrap().as_str()); }
        if let Some(v) = &self.platform { f.insert("platform", to_value(v).unwrap().as_str()); }
        if let Some(v) = &self.date_created_gte {
            f.insert(
                "date_created.secs_since_epoch",
                doc! {"$gte": v.duration_since(UNIX_EPOCH).unwrap().as_secs_f64() }
            );
        }
        if let Some(v) = &self.date_created_lte {
            f.insert(
                "date_created.secs_since_epoch",
                doc! {"$lte": v.duration_since(UNIX_EPOCH).unwrap().as_secs_f64() }
            );
        }
        f
    }
}
