use async_trait::async_trait;
use super::{Bot, errors::BotError};

#[async_trait]
pub trait SocialBotActions {
    async fn fetch_by_access_token(token: &str) -> Result<Bot, BotError>;
}
