use std::collections::HashMap;

use bson::Document;
use mongodb::Collection;
use serde::{Serialize, Deserialize };
use serde_json::to_value;

use crate::db::{DbQuery, SocialsDb, errors::DbError};

use super::SocialPlatform;

pub mod tests;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SourcePlatformInfo {
    platform: SocialPlatform,
    source_link: String
    // TODO
}

// TODO
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialSourceQuery {
    id: Option<bson::Uuid>,
    title: Option<String>
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

// impl FindById for SocialSource {}
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SocialSource {
    #[serde(default)]
    id: bson::Uuid,
    #[serde(default)]
    pub avatar: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub platforms: Vec<SourcePlatformInfo>
    // Vec<SourcePlatform>
}

impl SocialSource {
    pub async fn find_by_id(id: bson::Uuid, collection: Collection<SocialSource>) -> Result<Option<SocialSource>, DbError> {
        let query = SocialSourceQuery { id: Some(id), ..Default::default()};
        println!("query is {:#?}", query.collect_filters());
        let result = SocialsDb::find_one(query, collection).await;
        result
    }
}
