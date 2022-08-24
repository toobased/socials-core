use std::time::{SystemTime, Duration};

use log::info;
use serde::{Serialize, Deserialize};

use super::{TaskAction, BotTask};

// use super::{TaskAction, BotTask};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WatchTarget { 
    Video
}

impl Default for WatchTarget {
    fn default() -> Self { Self::Video }
}

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
    max_watch_spawn: u64
}

impl Default for WatchSettings {
    fn default() -> Self {
        Self {
            max_watch_spawn: 4
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WatchAction {
    pub target: WatchTarget,
    pub data: WatchTargetData,
    #[serde(default="WatchStats::default")]
    pub stats: WatchStats,
    #[serde(default="WatchSettings::default")]
    pub settings: WatchSettings
}

impl TaskAction for WatchAction {
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
        info!("Invoke `calc_next_time_run` {}: {:#?}", task.title, self.data);
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
        info!("Invoke `calc_need_do_now` {}: {:#?}", task.title, self.data);
        let max = self.settings.max_watch_spawn;
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
