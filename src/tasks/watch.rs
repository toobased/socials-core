use std::time::{SystemTime, Duration};

use async_trait::async_trait;
use log::{info, debug};
use serde::{Serialize, Deserialize};

use crate::{browser_core::BrowserCore, social::{SocialPlatform, vk_core::VkCore}, db::SocialsDb};

use super::{TaskAction, BotTask, TaskTarget, TaskActionType, errors::TaskError, BotTaskTypeQuery, BotTaskType};

// use super::{TaskAction, BotTask};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatchTargetData {
    pub watch_count: u64,
    pub watch_seconds: u64,
    pub resource_link: String,
    #[serde(default="WatchTargetData::default_time_spread")]
    pub time_spread: u64
}
impl Default for WatchTargetData {
    fn default () -> Self {
        Self {
            watch_count: 1,
            watch_seconds: 30,
            resource_link: String::from(""),
            time_spread: Self::default_time_spread()
        }
    }
}

impl WatchTargetData {
    pub fn default_time_spread () -> u64 { 300 }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WatchStats {
    pub watched_count: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatchSettings {
    pub take_screenshot: bool,
    pub max_watch_spawn: u64
}

impl Default for WatchSettings {
    fn default() -> Self {
        Self {
            take_screenshot: false,
            max_watch_spawn: 4
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WatchAction {
    pub target: TaskTarget,
    pub data: WatchTargetData,
    #[serde(default="WatchStats::default")]
    pub stats: WatchStats,
    #[serde(default="WatchSettings::default")]
    pub settings: WatchSettings
}

#[async_trait]
impl TaskAction for WatchAction {

    // TODO simplify, refactor
    async fn validate_limits(&self, platform: &SocialPlatform, db: &SocialsDb) -> Result<bool, TaskError> {
        let mut q = BotTaskTypeQuery::default();
        q.with_action_type(&self.action_type());
        let task_type = SocialsDb::find::<BotTaskType, BotTaskTypeQuery>(&q, &db.task_types()).await;
        match task_type {
            // HANDLE DB ERROR -> TaskError
            Err(e) => Err(e.into()),
            Ok(t) => {
                // NO RECORD FOR `SELF.ACTION_TYPE` FOUND, return true
                if t.items.len() == 0 {  return Ok(true) }
                let task_type = t.items.get(0).unwrap();
                let limits = task_type.get_target_platform_limits(&self.target, platform);
                match limits {
                    None => return Ok(true),
                    Some(l) => {
                        // CHECK COUNT LIMITS
                        if l.count_limit.is_some() {
                            let c = l.count_limit.unwrap();
                            if self.data.watch_count > c.into() {
                                return Err(TaskError::invalid_count_limit(
                                    Some(format!("allowed: {}, specified: {}", c, self.data.watch_count).as_str())
                                ))
                            }
                        }
                    }
                };
                Ok(true)
            }
        }
    }

    async fn validate_assign_data(&mut self, platform: &SocialPlatform) -> Result<bool, TaskError> {
        match platform {
            SocialPlatform::Vk => VkCore::validate_watch_data(self).await,
            _ => Ok(true)
        }
        // TODO check if post already present in task, get data from it
    }

    fn action_type(&self) -> TaskActionType { TaskActionType::Watch }
    fn target(&self) -> TaskTarget { self.target.clone() }
    fn use_browser(&self) -> bool { true }

    fn check_done(&self, task: &mut BotTask) -> bool {
        match self.stats.watched_count >= self.data.watch_count {
            true => {
                task.set_done();
                true
            },
            false => false
        }
    }

    fn calc_next_time_run(&self, task: &mut super::BotTask) {
        debug!("Invoke `calc_next_time_run` {}: {:#?}", task.title, self.data);
        self.check_done(task);
        if task.is_done() {  return }
        let action_process_time: u64 = self.data.watch_seconds;
        let now = SystemTime::now();
        let created = task.date_created;
        let time_spread = self.data.time_spread;
        let need_make = self.data.watch_count - self.stats.watched_count;
        let time_need = action_process_time * need_make;

        let time_passed = now.duration_since(created).unwrap().as_secs();
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
        /*
        if time_ratio > 0 {
            appender =  time_left / time_ratio;
        }
        */
        info!("{}s since created, spread: {}s, need: {}s, left: {}s, ratio: {}, next run in: {}s",
            time_passed, time_spread, time_need, time_left, time_ratio, appender);
    }
    fn calc_need_do_now(&self, task: &BotTask) -> u64 {
        debug!("Invoke `calc_need_do_now` {}: {:#?}", task.title, self.data);
        // let max = self.settings.max_watch_spawn;
        let max = BrowserCore::get_max_watch_spawn();
        let action_process_time: u64 = self.data.watch_seconds;
        let now = SystemTime::now();
        let created = task.date_created;
        let time_spread = self.data.time_spread;
        let need_make = self.data.watch_count - self.stats.watched_count;
        let time_need = action_process_time * need_make;

        let time_passed = now.duration_since(created).unwrap().as_secs();
        let time_left = {
            match time_spread > time_passed {
                true => time_spread - time_passed,
                false => 0
            }
        };
        let need_do = match time_left > 0 {
            true => time_need / time_left,
            false => need_make
        };
        info!("Need do now: {}, max: {}", need_do, max);
        match need_do > max {
            true => max,
            false => need_do
        }
    }
}
