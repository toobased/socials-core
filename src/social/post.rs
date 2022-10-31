use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use super::{attachments::SocialAttachmentType, SocialPlatform, vk_core::VkCore, errors::SocialError};

#[cfg(test)]
pub mod tests;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialPostMetric {
    pub count: u32,
    pub user_count: u32,
}

// TODO transform to trait to make generic?
// TODO if yes, what type will be passed in response
// SocialPost -> SocialPost
/*
*/
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialPost {
    pub id: bson::Uuid,
    pub owner_id: Option<String>,
    pub post_id: Option<String>,
    pub social_delayed_id: Option<String>,
    pub text: String,
    pub likes: SocialPostMetric,
   pub reposts: SocialPostMetric,
    pub views: SocialPostMetric,
    pub post_type: String,
    // TODO
    pub attachments: Vec<SocialAttachmentType>,
}

impl SocialPost {
    pub fn parse_data_from_url (p: &SocialPlatform, url: &str)
    -> Result<SocialPostParseData, SocialError> {
        // TODO handle normal error
        match p {
            SocialPlatform::Vk => VkCore::new().parse_data_from_url(url),
            _ => Err(SocialError::not_implemented(Some("parse_data_from_url")))
        }
    }

    pub async fn get_post_by_data(p: &SocialPlatform, d: &SocialPostParseData)
    -> Result<Self, SocialError> {
        match p {
            SocialPlatform::Vk => VkCore::new().get_post_by_data(d).await,
            _ => Err(SocialError::not_implemented(Some("get_post_by_data not implemented")))
        }
    }
    pub async fn get_post_by_url(p: &SocialPlatform, url: &str)
        -> Result<SocialPost, SocialError> {
        match p {
            SocialPlatform::Vk => VkCore::new().get_post_by_url(url).await,
            _ => Err(SocialError::not_implemented(Some("get_post_by_url not implemented")))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SocialPostParseData {
    pub owner_id: String,
    pub post_id: String,
}

#[async_trait]
pub trait SocialPostActions {
    fn parse_data_from_url (&self, url: &str)
        -> Result<SocialPostParseData, SocialError>;
    async fn get_post_by_data(&self, d: &SocialPostParseData)
        -> Result<SocialPost, SocialError>;
    async fn get_post_by_url(&self, url: &str)
        -> Result<SocialPost, SocialError> {
        match self.parse_data_from_url(url) {
            Err(e) => Err(e),
            Ok(d) => self.get_post_by_data(&d).await
        }
    }
}

