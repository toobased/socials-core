use std::time::Duration;

use fantoccini::Locator;
use log::info;
use serde::{Serialize, Deserialize};

use crate::{tasks::{BotTask, like::LikeAction, watch::WatchAction, TaskActionEnum, TaskAction, errors::TaskError}, browser_core::BrowserCore, db::SocialsDb};


use async_trait::async_trait;

pub mod errors;

pub mod source;
pub mod post;
pub mod attachments;

// social cores
pub mod yt_core;
pub mod vk_core;
pub mod ok_core;
pub mod dzen_core;

pub trait SocialCoreConfig {
    fn video_play_btn_cls(&self) -> Option<&String> { None }
}

#[async_trait]
pub trait SocialCore {
    type CoreConfig: SocialCoreConfig + Sync;

    fn info(&self) -> String;
    fn config(&self) -> &Self::CoreConfig;

    async fn make_action(&self, task: &mut BotTask, db: &SocialsDb) {
        let action = task.action.clone();
        match action {
            TaskActionEnum::LikeAction(a) => self.like(a, task, db).await,
            TaskActionEnum::WatchAction(a) => self.watch(a, task, db).await,
            _ => info!("{:#?} dont match any action variants. Shouldnt happen.", action),
        };
    }

    async fn like(&self, _action: LikeAction, _task: &mut BotTask, _db: &SocialsDb) {
        info!("not implemented yet?");
    }

    async fn watch_task(&self, action: &WatchAction) -> Result<(), TaskError> {
        info!("Run watch_task from trait!");
        let config = self.config();
        let browser = BrowserCore::init().await;
        let client = &browser.client;
        let link = &action.data.resource_link;
        let watch_seconds = action.data.watch_seconds;
        let play_btn_cls = config.video_play_btn_cls();

        match client.goto(link).await {
            Ok(_) => {},
            Err(_) => {
                browser.close().await;
                return Err(TaskError::incorrect_link(link))
            }
        }

        // click on play btn if its required
        match play_btn_cls {
            None => {},
            Some(btn_cls) => {
                match client
                    .find(Locator::Css(btn_cls)).await {
                        Err(_) => {
                            browser.close().await;
                            return Err(TaskError::action_error(
                                Some("Cant find video play btn element"),
                                Some("Prob. need to improve social interaction")
                            ))
                        }
                        Ok(e) => {
                            match e.click().await {
                                Ok(_) => {},
                                Err(_) => {
                                    browser.close().await;
                                    return Err(TaskError::element_click(Some("Cant click on video play btn")))
                                }
                            }
                        }
                    }
            }
        };

        info!("Fall asleep for {}s", watch_seconds);
        tokio::time::sleep(Duration::from_secs(watch_seconds)).await;
        info!("Closing client");
        browser.close().await;
        Ok(())
    }

    async fn watch(&self, action: WatchAction, task: &mut BotTask, db: &SocialsDb) {
        // info!("Run watch action from trait. Not implemented yet. Core: {}", self.info())
        info!("Run watch action from trait");
        let need_do = action.calc_need_do_now(task);
        let actions = (0..need_do).map(|_| self.watch_task(&action));
        let results = futures::future::join_all(actions).await;

        for result in results.iter() {
            let r = result.clone();
            match r {
                Ok(_) => {},
                Err(e) => { task.set_error(e); }
            }
        }

        if !task.has_error() {
            task.get_fresh(&db).await.unwrap();
            // TODO fix cloning
            let fresh_action = task.action.clone();
            match fresh_action {
                TaskActionEnum::WatchAction(mut action) => {
                    action.stats.watched_count += need_do;
                    action.calc_next_time_run(task);
                    task.action = TaskActionEnum::WatchAction(action);
                },
                _ => {}
            }
        }
        task.update_db(db).await
            .expect("Cant update task in db");
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SocialPlatform { Unspecified, Vk, Ok, Instagram, Youtube, Dzen }

impl Default for SocialPlatform {
    fn default() -> Self { Self::Unspecified }
}
