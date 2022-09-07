// use std::collections::HashMap;

use bson::Document;
use mongodb::Collection;
use serde::{Serialize, Deserialize };
// use serde_json::to_value;

use crate::db::{DbQuery, SocialsDb, errors::DbError};

use super::SocialPlatform;

pub mod tests;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SourcePlatformInfo {
    pub platform: SocialPlatform,
    pub source_link: String
    // TODO
}

// TODO
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialSourceQuery {
    pub id: Option<bson::Uuid>,
    pub title: Option<String>
}

// TODO
impl DbQuery for SocialSourceQuery {
    fn collect_filters(&self) -> Document {
        let mut f = Document::new();
        if let Some(i) = &self.id {
            f.insert("id", i);
        }
        if let Some(i) = &self.title {
            f.insert("name", i);
        }
        f
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialSourceCreate {
    #[serde(default)]
    pub avatar: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub platforms: Vec<SourcePlatformInfo>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialSource {
    id: bson::Uuid,
    #[serde(default)]
    pub avatar: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub platforms: Vec<SourcePlatformInfo>
}

impl SocialSource {
    pub async fn find_by_id(id: bson::Uuid, collection: Collection<SocialSource>) -> Result<Option<SocialSource>, DbError> {
        let query = SocialSourceQuery { id: Some(id), ..Default::default()};
        println!("query is {:#?}", query.collect_filters());
        let result = SocialsDb::find_one(&query, &collection).await;
        result
    }
    pub fn update_with(&mut self, s: SocialSourceCreate) {
        self.avatar = s.avatar;
        self.name = s.name;
        self.description = s.description;
        self.platforms = s.platforms;
    }
}

impl From<SocialSourceCreate> for SocialSource {
    fn from(s: SocialSourceCreate) -> Self {
        Self {
            id: bson::Uuid::new(),
            avatar: s.avatar,
            name: s.name,
            description: s.description,
            platforms: s.platforms
        }
    }
}
