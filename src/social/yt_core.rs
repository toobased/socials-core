use async_trait::async_trait;
use super::{SocialCore, SocialCoreConfig};

// locale
pub mod tests;

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

impl SocialCoreConfig for YtCoreConfig {
    fn video_play_btn_cls(&self) -> Option<&String> { Some(&self.video_play_btn_cls) }
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

impl YtCore {
    pub fn new () -> Self { Self::default() }

}

#[async_trait]
impl SocialCore for YtCore {
    type CoreConfig = YtCoreConfig;

    fn config(&self) -> &YtCoreConfig { &self.config }

    fn info(&self) -> String {
    "YtCore".to_string()
    }
}
