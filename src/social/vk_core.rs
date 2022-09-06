use crate::{tasks::{like::LikeAction, BotTask}, db::SocialsDb};

use super::{SocialCore, SocialCoreConfig};

pub struct VkCoreConfig { }

impl Default for VkCoreConfig {
    fn default() -> Self {
        Self {}
    }
}

impl SocialCoreConfig for VkCoreConfig {}

pub struct VkCore {
    pub config: VkCoreConfig
}

impl VkCore {
    pub fn new () -> Self { Self::default() }
}

impl Default for VkCore {
    fn default() -> Self {
        Self {
            config: VkCoreConfig::default()
        }
    }
}


impl SocialCore for VkCore {
    type CoreConfig = VkCoreConfig;

    fn config(&self) -> &VkCoreConfig { &self.config }

    fn info(&self) -> String {
        "VkCore".to_string()
    }
    fn like(&self, _action: LikeAction, _task: &mut BotTask, _db: &SocialsDb) {
        println!("run for vk platform")
    }
}
