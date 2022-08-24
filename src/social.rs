use fantoccini::Locator;
use log::info;
use serde::{Serialize, Deserialize};

use crate::{tasks::{BotTask, TaskActionType, TaskAction, TaskActionEnum, like::LikeAction, watch::WatchAction, errors::TaskError}, browser_core::BrowserCore, db::SocialsDb};
use std::{fmt::Display, thread, time::Duration};
use async_trait::async_trait;

pub mod source;

#[async_trait]
pub trait SocialCore {
    // TODO replace to display
    fn info(&self) -> String;
    /*
    fn make_action(&self, task: &mut BotTask) {
        match task.action_type {
            TaskActionType::Like => println!("need like"),
            // task.actions.like.do_stuff(),
            _ => println!("unimplemented shit")
        }
    }
    */
    async fn make_action(&self, task: &mut BotTask) {
        let mut action = task.action.clone();
        match action {
            TaskActionEnum::LikeAction(a) => self.like(a, task),
            TaskActionEnum::WatchAction(a) => self.watch(a, task).await,
            _ => (),
        };
    }

    fn like(&self, action: LikeAction, task: &mut BotTask) {}

    async fn watch(&self, action: WatchAction, task: &mut BotTask) {
        info!("Run watch action from trait. Not implemented yet. Core: {}", self.info())
    }
}

pub struct VkCore {}
pub struct OkCore {}

#[derive(Clone)]
pub struct YtCoreConfig {
    video_play_btn_cls: String
}

impl Default for YtCoreConfig {
    fn default () -> Self {
        Self {
            video_play_btn_cls: ".ytp-large-play-button".to_string()
        }
    }
}

pub struct YtCore {
    config: YtCoreConfig
}

impl Default for YtCore {
    fn default() -> Self {
        Self {
            config: YtCoreConfig::default()
        }
    }
}

impl VkCore {
    pub fn new () -> Self { Self {} }
}

impl OkCore {
    pub fn new () -> Self { Self {} }
}

impl YtCore {
    pub fn new () -> Self { Self::default() }
    async fn watch_task(action: &WatchAction, config: &YtCoreConfig) -> Result<(), TaskError> {
        let browser = BrowserCore::init().await;
        let client = &browser.client;
        let link = &action.data.resource_link;
        let watch_seconds = action.data.watch_seconds;
        info!("Run watch action with {}", "yt core");
        match client.goto(link).await {
            Ok(_) => {},
            Err(_) => {
                browser.close().await;
                return Err(TaskError::incorrect_link(link));
            }
        }
        let play_btn = match client
            .find(Locator::Css(&config.video_play_btn_cls)).await {
                Ok(e) => e,
                Err(_) => {
                    browser.close().await;
                    return Err(TaskError::action_error(
                        Some("Cant find video play btn element".to_string()),
                        Some("Prob. need to improve social interaction".to_string())
                    ))
                },
            };
        match play_btn.click().await {
            Ok(_) => {},
            Err(_) => {
                browser.close().await;
                return Err(TaskError::element_click(Some("Cant click on yt video play btn")))
            }
        }
        info!("Fall asleep for {}s", watch_seconds);
        tokio::time::sleep(Duration::from_secs(watch_seconds)).await;
        info!("Closing client");
        browser.close().await;
        // browser.close();
        // browser.client.close().await;
        // client.close().await.expect("Failed by closing client");
        Ok(())
    }
}

#[async_trait]
impl SocialCore for YtCore {
    fn info(&self) -> String {
    "YtCore".to_string()
    }

    async fn watch(&self, action: WatchAction, task: &mut BotTask) {
        info!("Run generic watch");
        let need_do = action.calc_need_do_now(task);
        let actions = (0..need_do).map(|_| YtCore::watch_task(&action, &self.config));
        let results = futures::future::join_all(actions).await;

        for result in results.iter() {
            let r = result.clone();
            match r {
                Ok(_) => {},
                Err(e) => { task.set_error(e) }
            }
        }

        if !task.has_error() {
            let mut a = action.clone();
            a.stats.watched_count += need_do;
            a.calc_next_time_run(task);
            task.action = TaskActionEnum::WatchAction(a);
        }

    }
}

impl SocialCore for OkCore {
  fn info(&self) -> String {
    "OkCore".to_string()
  }
  fn like(&self, action: LikeAction, task: &mut BotTask) {
      println!("run for ok platform")
  }
}

impl SocialCore for VkCore {
  fn info(&self) -> String {
    "VkCore".to_string()
  }
  fn like(&self, action: LikeAction, task: &mut BotTask) {
      println!("run for vk platform")
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SocialPlatform {
    Unspecified,
    Vk,
    Ok,
    Instagram,
    Youtube,
}

impl Default for SocialPlatform {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl SocialPlatform { }

/*
impl SocialPlatform {
    pub fn make_action(&self, task: &mut BotTask) {
        let vk_core = VkCore::new();
        let ok_core = OkCore::new();
        match self {
            Self::Vk => vk_core.make_action(task),
            _ => ok_core.make_action(task)
        }
    }
}
*/
