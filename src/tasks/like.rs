use std::time::SystemTime;

use log::info;
use serde::{Serialize, Deserialize};

use super::{TaskAction, BotTask, TaskTarget};

// use super::{TaskAction, BotTask};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TaskTargetData {
    like_count: i32,
    like_random_threshold: i32,
    /// for bulk like account / group
    last_items_check_count: i32,
    resource_link: String,
    date_finish: Option<SystemTime>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeStats {
    like_count: i32,
    processed_posts_ids: Vec<String>
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeSettings {
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
// TODO make parse some field auto on json parsing
pub struct LikeAction {
    #[serde(default="TaskTarget::default")]
    pub target: TaskTarget,
    pub data: TaskTargetData,
    pub stats: LikeStats,
    pub settings: LikeSettings
}

impl TaskAction for LikeAction {
    fn calc_next_time_run(&self, _task: &mut BotTask) {
        info!("check need run for like post");
    }
    fn calc_need_do_now(&self, _task: &BotTask) -> u64 {
        10
    }

    fn check_done(&self, task: &mut BotTask) -> bool {
        let done = self.stats.like_count >= self.data.like_count;
        match done {
            true => {
                task.set_done();
                done
            },
            false => false
        }
    }
}
