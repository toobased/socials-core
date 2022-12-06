use async_trait::async_trait;
use log::info;
use vk_client::{wall::{response::WallPost, types::{WallPostLike, WallPostRepost, WallPostView}}, client::VkClient};

use crate::social::{post::{SocialPost, SocialPostMetric, SocialPostActions, SocialPostParseData}, errors::SocialError};

use super::VkCore;

impl From<WallPostLike> for SocialPostMetric {
    fn from(v: WallPostLike) -> Self { SocialPostMetric { count: v.count, user_count: v.user_likes.into() } }
}
impl From<WallPostRepost> for SocialPostMetric {
    fn from(v: WallPostRepost) -> Self { SocialPostMetric { count: v.count, user_count: v.user_reposted.into() } }
}
impl From<WallPostView> for SocialPostMetric {
    fn from(v: WallPostView) -> Self { SocialPostMetric { count: v.count, ..Default::default() } }
}

impl From<WallPost> for SocialPost {
    fn from(v: WallPost) -> Self {
        SocialPost {
            id: bson::Uuid::new(),
            post_id: Some(v.id.to_string()),
            social_delayed_id: Some(v.postponed_id.unwrap_or(0).to_string()),
            owner_id: Some(v.owner_id.to_string()),
            text: v.text,
            likes: v.likes.into(),
            reposts: v.reposts.into(),
            views: v.views.into(),
            attachments: v.attachments.into_iter().map(|v| {v.into()} ).collect(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl SocialPostActions for VkCore {
    fn parse_data_from_url (&self, url: &str)
    -> Result<SocialPostParseData, SocialError> {
        // https://vk.com/kf_films?w=wall-211982694_1403
        let msg = format!("VkCore fail to parse {}", url);
        let e = Err(SocialError::parse_post_url(Some(&msg)));

        match url.split("wall-").last() {
            None => return e,
            Some(k) => {
                match k.split('&').into_iter().collect::<Vec<&str>>().get(0) {
                    None => e,
                    Some(v) => {
                        let res: Vec<&str> = v.split("_").into_iter().collect();
                        let owner_id = res.get(0);
                        let item_id = res.get(1);
                        if owner_id.is_none() || item_id.is_none() { return e }
                        let d = SocialPostParseData {
                            owner_id: owner_id.unwrap().to_string(),
                            post_id: item_id.unwrap().to_string()
                        };
                        Ok(d)
                    }
                }

            }
        }
    }

    async fn get_post_by_data(&self, d: &SocialPostParseData)
    -> Result<SocialPost, SocialError> {
        info!("[VkCore `get_post_by_data`]");
        let msg = format!("VkCore fail to get post {:#?}", d);
        let e = Err(SocialError::get_post(Some(&msg)));
        // FIXME change to add - only when its group
        let data = format!("-{}_{}", d.owner_id, d.post_id);
        let q =  vk_client::wall::query::GetByIdQuery {
            posts: data,
            ..Default::default()
        };
        let client = VkClient::init_admin();
        let res = vk_client::wall::get_by_id(&client, q).await;
        // info!("[VkCore `get_post_by_data`] res is {:#?}", res);
        match res {
            Err(_) => e,
            Ok(r) => match r.get(0) {
                None => e,
                Some(item) => Ok(item.clone().into())
            }
        }
    }
}
