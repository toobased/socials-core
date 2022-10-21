use std::time::{SystemTime, Duration};

use log::info;
use serde::{Serialize, Deserialize};

use super::{TaskAction, BotTask, TaskTarget, TaskActionEnum};

#[cfg(test)]
pub mod tests;

// use super::{TaskAction, BotTask};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeTargetData {
    pub like_count: i32,
    pub like_random_threshold: i32,
    /// for bulk like account / group
    last_items_check_count: i32,
    pub owner_id: Option<String>,
    pub item_id: Option<String>,
    resource_link: String,
    #[serde(default="LikeTargetData::default_time_spread")]
    pub time_spread: u64
    // date_finish: Option<SystemTime>
}

impl LikeTargetData {
    pub fn default_time_spread () -> u64 { 600 }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeStats {
    pub like_count: i32,
    pub processed_posts_ids: Vec<String>
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeSettings {
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
// TODO make parse some field auto on json parsing
pub struct LikeAction {
    #[serde(default="TaskTarget::default")]
    pub target: TaskTarget,
    pub data: LikeTargetData,
    pub stats: LikeStats,
    pub settings: LikeSettings
}

impl TryFrom<TaskActionEnum> for LikeAction {
    type Error = &'static str;
    fn try_from(a: TaskActionEnum) -> Result<Self, Self::Error> {
        match a {
            TaskActionEnum::LikeAction(a) => Ok(a),
            _ => Err("failed to convert action to like action")
        }
    }
}

impl TaskAction for LikeAction {

    fn calc_next_time_run(&self, task: &mut BotTask) {
        info!("Invoke `calc_next_time_run` {}: {:#?}", task.title, self.data);
        self.check_done(task);
        if task.is_done() { return };
        let now = SystemTime::now();
        let created = task.date_created;
        let time_spread = self.data.time_spread;
        let need_make = self.data.like_count - self.stats.like_count;
        let time_passed = now.duration_since(created).unwrap().as_secs();
        let time_need = u64::try_from(1 * need_make).unwrap();

        let time_left = {
            match time_spread > time_passed {
                true => time_spread - time_passed,
                false => 0
            }
        };
        let time_ratio: u64 = time_left / time_need;
        let appender = time_ratio;
        // update task next_time_run
        task.next_run_time = SystemTime::now().checked_add(Duration::from_secs(appender));
        info!("{}s since created, spread: {}s, need: {}s, left: {}s, ratio: {}, next run in: {}s",
            time_passed, time_spread, time_need, time_left, time_ratio, appender);
    }

    fn calc_need_do_now(&self, _task: &BotTask) -> u64 { 10 }

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
