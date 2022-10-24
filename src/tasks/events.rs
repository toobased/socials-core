use std::time::SystemTime;

use serde::{Serialize, Deserialize};

use crate::social::SocialPlatform;

use super::{TaskTarget, TaskActionType};

pub mod query;

#[derive(Serialize, Deserialize, Debug, Default)]
struct ActionEventPayload {
    count_amount: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
struct ActionEvent {
    pub id: Option<bson::Uuid>,
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
