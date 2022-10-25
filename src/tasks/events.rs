use std::time::SystemTime;

use serde::{Serialize, Deserialize};

use crate::{social::SocialPlatform, db::{SocialsDb, DbActions}};

use self::query::ActionEventQuery;

use super::{TaskTarget, TaskActionType, BotTask };

pub mod query;

#[cfg(test)]
pub mod tests;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ActionEventPayload {
    pub count_amount: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionEvent {
    pub id: bson::Uuid,
    pub task_id: Option<bson::Uuid>,
    pub bot_id: Option<bson::Uuid>,

    #[serde(default="SystemTime::now")]
    pub date_created: SystemTime,

    #[serde(default="ActionEventPayload::default")]
    pub payload: ActionEventPayload,

    pub action_type: TaskActionType,
    pub task_target: TaskTarget,
    pub platform: SocialPlatform
}

impl Default for ActionEvent {
    fn default() -> Self {
        Self {
            id: bson::Uuid::new(),
            task_id: None,
            bot_id: None,
            date_created: SystemTime::now(),
            payload: ActionEventPayload::default(),
            action_type: TaskActionType::Dummy,
            task_target: TaskTarget::Dummy,
            platform: SocialPlatform::Unspecified
        }
    }
}

impl ActionEvent {

    pub fn from_task(task: &BotTask) -> Self {
        Self {
            id: bson::Uuid::new(),
            task_id: Some(task.id),
            bot_id: None,
            date_created: SystemTime::now(),
            payload: ActionEventPayload::default(),
            action_type: task.action_type.clone(),
            task_target: task.action.target(),
            platform: task.platform
        }
    }

    pub fn set_bot_id(&mut self, v: bson::Uuid) -> &mut Self { self.bot_id = Some(v); self }
    pub fn set_amount(&mut self, v: u32) -> &mut Self { self.payload.count_amount = Some(v); self }
}

impl DbActions for ActionEvent {
    type Query = ActionEventQuery;
    fn get_collection (&self, db: &SocialsDb) -> mongodb::Collection<ActionEvent> {
        db.action_events()
    }
    fn get_id (&self) -> bson::Uuid { self.id }
}
