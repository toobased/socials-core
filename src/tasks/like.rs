use std::time::{SystemTime, Duration};

use async_trait::async_trait;
use log::{info, debug};
use serde::{Serialize, Deserialize};

use crate::{bots::BotLimitSleep, utils::pretty_duration, social::{SocialPlatform, vk_core::VkCore}};

use super::{TaskAction, BotTask, TaskTarget, TaskActionEnum, TaskActionType, errors::TaskError, ActionExtra};

#[cfg(test)]
pub mod tests;

// use super::{TaskAction, BotTask};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeTargetData {
    pub like_count: u64,
    pub like_random_threshold: u64,
    /// for bulk like account / group
    pub last_items_check_count: u64,
    pub owner_id: Option<String>,
    pub item_id: Option<String>,
    pub resource_link: Option<String>,
    #[serde(default="LikeTargetData::default_time_spread")]
    pub time_spread: u64
    // date_finish: Option<SystemTime>
}

impl LikeTargetData {
    pub fn default_time_spread () -> u64 { 600 }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeStats {
    pub like_count: u64,
    pub processed_posts_ids: Vec<String>,
    #[serde(default="Vec::default")]
    pub bots_used: Vec<bson::Uuid>
}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikeSettings {
    pub testing_check_liked: bool,
    pub testing_add_used: bool
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
// TODO make parse some field auto on json parsing
pub struct LikeAction {
    #[serde(default="TaskTarget::default")]
    pub target: TaskTarget,
    pub data: LikeTargetData,
    #[serde(default="LikeStats::default")]
    pub stats: LikeStats,
    #[serde(default="LikeSettings::default")]
    pub settings: LikeSettings,
    #[serde(default="ActionExtra::default")]
    pub extra: ActionExtra,
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

impl LikeAction {
    pub fn add_used_bot(&mut self, bot_id: &bson::Uuid) -> &mut Self {
        debug!("add {} to used", bot_id);
        self.stats.bots_used.push(bot_id.clone()); self
    }

    pub fn is_testing_check_liked (&self) -> bool { return self.settings.testing_check_liked }
    pub fn is_testing_add_used (&self) -> bool { return self.settings.testing_add_used }
}

#[async_trait]
impl TaskAction for LikeAction {

    async fn validate_assign_data(&mut self, platform: &SocialPlatform) -> Result<bool, TaskError> {
        match platform {
            SocialPlatform::Vk => VkCore::validate_like_data(self).await,
            _ => Ok(true)
        }
        // TODO check if post already present in task, get data from it
    }

    fn bot_assign_sleep(&self, bot: &mut crate::bots::Bot, sleep: Duration) {
        let sleep_until = SystemTime::now().checked_add(sleep).unwrap();
        info!("[Like Sleep] set sleep {} for {}", bot.id, pretty_duration(sleep));
        bot.actions_rest.like = Some(sleep_until);
    }
    fn action_type(&self) -> TaskActionType { TaskActionType::Like }
    fn target(&self) -> TaskTarget { self.target.clone() }

    // bot sleep times
    fn bot_min_sleep(&self) -> Duration {
        // 2min sleep
        Duration::from_secs(120)
    }
    fn bot_1hr_limit_sleep(&self) -> BotLimitSleep {
        // 1hr sleep
        BotLimitSleep { limit: 4, sleep: Duration::from_secs(3600) }
    }
    fn bot_24hr_limit_sleep(&self) -> BotLimitSleep {
        // 5hrs sleep
        BotLimitSleep { limit: 15, sleep: Duration::from_secs(18000) }
    }

    fn calc_next_time_run(&self, task: &mut BotTask) {
        debug!("Invoke `calc_next_time_run` {}: {:#?}", task.title, self.data);
        let time_now = SystemTime::now();
        let date_created = task.date_created;
        let time_spread = self.data.time_spread;
        if self.stats.like_count.gt(&self.data.like_count) {
            task.next_run_time = None;
            task.set_done();
            return
        }
        let need_make = self.data.like_count - self.stats.like_count;
        if need_make.eq(&0) {
            task.next_run_time = None;
            task.set_done();
            return
        }
        let secs_elapsed = time_now.duration_since(date_created).unwrap().as_secs();
        let secs_left = match secs_elapsed > time_spread {
            true => 0,
            false => time_spread - secs_elapsed
        };
        // let max_make_per_step = 5; // TODO move to config
        // let min_sleep_step = 10;
        // let max_make = std::cmp::min(need_make, max_make_per_step);
        let mut sleep_step = match secs_left.eq(&0) {
            true => 1,
            false => secs_left / need_make
        };
        if sleep_step.lt(&0) { sleep_step = 1; }
        task.next_run_time = SystemTime::now().checked_add(Duration::from_secs(sleep_step));
    }

    fn calc_need_do_now(&self, task: &BotTask) -> u64 {
        debug!("Invoke `calc_need_do_now` {}: {:#?}", task.title, self.data);
        let time_now = SystemTime::now();
        let date_created = task.date_created;
        let time_spread = self.data.time_spread;
        if self.stats.like_count.gt(&self.data.like_count) { return 0 }
        let need_make = self.data.like_count - self.stats.like_count;
        if need_make.eq(&0) { return 0 }
        let secs_elapsed = time_now.duration_since(date_created).unwrap().as_secs();
        let secs_left = match secs_elapsed > time_spread {
            true => 0,
            false => time_spread - secs_elapsed
        };
        let max_make_per_step = 5; // TODO move to config
        let min_sleep_step = 10;
        //
        let max_make = std::cmp::min(need_make, max_make_per_step);
        //
        let mut sleep_step = match secs_left.eq(&0) {
            true => 1,
            false => secs_left / need_make
        };
        if sleep_step.eq(&0) { sleep_step = 1; }
        info!("{}", min_sleep_step <= sleep_step);
        match min_sleep_step.le(&sleep_step) {
            true => return 1,
            false => {
                info!("{} {}", min_sleep_step, sleep_step);
                let need_do_required = min_sleep_step / sleep_step;
                return std::cmp::min(need_do_required, max_make)
            }
        }
    }

    fn check_done(&self, task: &mut BotTask) -> bool {
        let done = self.stats.like_count >= self.data.like_count;
        match done {
            true => { task.set_done(); done },
            false => false
        }
    }
}
