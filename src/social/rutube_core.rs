use async_trait::async_trait;
use super::{SocialCore, SocialCoreConfig};

#[cfg(test)]
pub mod tests;

#[derive(Clone, Default)]
pub struct RutubeCoreConfig { }

impl SocialCoreConfig for RutubeCoreConfig {
    // fn video_play_btn_cls(&self) -> Option<&String> { Some(&self.video_play_btn_cls) }
}

pub struct RutubeCore {
    config: RutubeCoreConfig
}

impl Default for RutubeCore {
    fn default() -> Self {
        Self {
            config: RutubeCoreConfig::default()
        }
    }
}

impl RutubeCore {
    pub fn new () -> Self { Self::default() }
}

#[async_trait]
impl SocialCore for RutubeCore {
    type CoreConfig = RutubeCoreConfig;


    fn config(&self) -> &RutubeCoreConfig { &self.config }

    fn info(&self) -> String {
    "RutubeCore".to_string()
    }
}
