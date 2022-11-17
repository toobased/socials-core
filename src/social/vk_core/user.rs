use std::time::SystemTime;

use async_trait::async_trait;
use vk_client::{client::response::VkError, users::response::VkUser};
use crate::{bots::{actions::SocialBotActions, errors::{BotError, BotErrorKind}, Bot, BotActionsRest, BotStatus, BotPlatformData, BotExtra}, social::SocialPlatform};
use super::VkCore;

#[async_trait]
impl SocialBotActions for VkCore {
    async fn fetch_by_access_token(token: &str) -> Result<Bot, BotError> {
        let client = VkCore::make_client(token);
        match vk_client::users::get(&client, None).await {
            Ok(mut r) => match r.get_mut(0) {
                Some(user) => Ok(Bot::from(user)),
                None => Err(BotError::common(Some("Vk fetch by access token, no user in response"), None))
            },
            Err(e) => Err(BotError::from(e))
        }
    }
}

impl From<&mut VkUser> for Bot {
    fn from(v: &mut VkUser) -> Self {
        let status = match v.is_closed {
            true => BotStatus::Banned,
            false => BotStatus::default()
        };
        return Self {
            id: bson::Uuid::new(),
            social_id: Some(v.id.to_string()),
            username: String::from(""),
            password: None,
            access_token: None,
            // times
            date_created: SystemTime::now(),
            date_updated: SystemTime::now(),
            last_used: None,
            rest_until: None,
            actions_rest: BotActionsRest::default(),
            // eof times
            platform: SocialPlatform::Vk,
            status,
            created_source: None,
            platform_data: BotPlatformData::default(),
            extra: BotExtra::default(),
            error: None,
            gender: None
        }
    }
}

impl From<VkError> for BotError {
    fn from(v: VkError) -> Self {
        return Self::new(
            BotErrorKind::Common,
            Some(&v.merge_msg()),
            Some(v.log.unwrap_or(String::from("")).as_str())
        )
    }
}
