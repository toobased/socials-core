use async_trait::async_trait;

use crate::{tasks::{like::LikeAction, BotTask}, db::SocialsDb};

use super::{SocialCore, SocialCoreConfig};

pub mod tests;

pub struct DzenCoreConfig {
    pub watch_video_btn_cls: String
}

impl Default for DzenCoreConfig {
    fn default() -> Self {
        Self {
            watch_video_btn_cls: "empty_here".to_string()
        }
    }
}

impl SocialCoreConfig for DzenCoreConfig {}

pub struct DzenCore {
    pub config: DzenCoreConfig
}

impl Default for DzenCore {
    fn default() -> Self {
        Self {
            config: DzenCoreConfig::default()
        }
    }
}

impl DzenCore {
    pub fn new () -> Self { Self::default() }
}

#[async_trait]
impl SocialCore for DzenCore {
    type CoreConfig = DzenCoreConfig;

    fn config(&self) -> &DzenCoreConfig { &self.config }

    fn info(&self) -> String { "DzenCore".to_string() }
    async fn like(&self, _action: LikeAction, _task: &mut BotTask, _db: &SocialsDb) {
        println!("run for dzen platform")
    }

    /*
    async fn watch_task(&self, _action: &WatchAction) -> Result<(), TaskError> {
        info!("Run watch_task from DzenCore");
        Err(TaskError::dummy())
    }
    */

}
