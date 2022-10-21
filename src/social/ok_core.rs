use async_trait::async_trait;

use crate::{tasks::{like::LikeAction, BotTask}, db::SocialsDb};

use super::{SocialCore, SocialCoreConfig};

pub struct OkCoreConfig { }

impl Default for OkCoreConfig {
    fn default() -> Self { Self {} }
}

impl SocialCoreConfig for OkCoreConfig {}

pub struct OkCore {
    pub config: OkCoreConfig
}

impl OkCore {
    pub fn new () -> Self { Self::default() }
}

impl Default for OkCore {
    fn default() -> Self {
        Self {
            config: OkCoreConfig::default()
        }
    }
}

#[async_trait]
impl SocialCore for OkCore {
    type CoreConfig = OkCoreConfig;

    fn config(&self) -> &OkCoreConfig { &self.config }

    fn info(&self) -> String {
        "OkCore".to_string()
    }
    async fn like(&self, _action: LikeAction, _task: &mut BotTask, _db: &SocialsDb) {
          println!("run for ok platform")
    }
}
