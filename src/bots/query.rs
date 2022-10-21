use bson::{Document, doc};
use mongodb::options::{FindOptions, FindOneOptions};
use serde::{Serialize, Deserialize};
use serde_json::to_value;

use crate::{social::SocialPlatform, db::DbQuery};

use super::BotStatus;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BotQuery {
    pub id: Option<bson::Uuid>,
    pub status: Option<BotStatus>,
    pub platform: Option<SocialPlatform>,
    pub sort_by_created_date: Option<i32>,
    pub sort_by_updated_date: Option<i32>,
    pub sort_by_last_used: Option<i32>,
    pub has_token: Option<bool>,
    pub skip: Option<u64>,
    pub limit: Option<i64>,
}

impl BotQuery {
    pub fn new() -> Self { Self::default() }
    pub fn is_ready(&mut self) -> &mut Self { self.status = Some(BotStatus::Ready); self }
    pub fn with_platform(&mut self, p: SocialPlatform) -> &mut Self { self.platform = Some(p); self }

    // time
    pub fn top_old_created(&mut self) -> &mut Self { self.sort_by_created_date = Some(1); self }
    pub fn top_old_updated(&mut self) -> &mut Self { self.sort_by_updated_date = Some(1); self }
    pub fn top_old_used(&mut self) -> &mut Self { self.sort_by_last_used = Some(1); self }
    pub fn top_new_created(&mut self) -> &mut Self { self.sort_by_created_date = Some(-1); self }
    pub fn top_new_updated(&mut self) -> &mut Self { self.sort_by_updated_date = Some(-1); self }
    pub fn top_new_used(&mut self) -> &mut Self { self.sort_by_last_used = Some(-1); self }

    // fields
    pub fn has_token (&mut self) -> &mut Self { self.has_token = Some(true); self }

    // options
    pub fn limit (&mut self, v: i64) -> &mut Self { self.limit = Some(v); self }
}

impl DbQuery for BotQuery {
    fn collect_filters(&self) -> bson::Document {
        let mut f = Document::new();
        if let Some(v) = &self.id { f.insert("id", v); }
        if let Some(v) = &self.status { f.insert("status", to_value(v).unwrap().as_str()); }
        if let Some(v) = &self.platform { f.insert("platform", to_value(v).unwrap().as_str()); }
        if let Some(_v) = &self.has_token { f.insert("access_token", doc! { "$ne": bson::Bson::Null } ); }
        f
    }
    fn collect_sorting(&self) -> Document {
        let mut f = Document::new();
        if let Some(v) = &self.sort_by_created_date { f.insert("date_created", v); }
        if let Some(v) = &self.sort_by_updated_date { f.insert("date_updated", v); }
        if let Some(v) = &self.sort_by_last_used { f.insert("last_used", v); }
        f
    }

    fn collect_options(&self) -> FindOptions {
        let mut f = FindOptions::default();
        f.skip = self.skip;
        f.limit = self.limit;
        f.sort = Some(self.collect_sorting());
        f
    }

    fn collect_one_options(&self) -> FindOneOptions {
        let mut f = FindOneOptions::default();
        f.sort = Some(self.collect_sorting());
        f
    }
}
