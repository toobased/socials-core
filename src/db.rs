// #[cfg(test)]
// pub mod tests;

use std::{borrow::Borrow, env};

use async_trait::async_trait;
use bson::Document;
use log::debug;
use mongodb::{Collection, options::{ClientOptions, FindOptions, FindOneOptions }, Database};
use futures::stream::TryStreamExt;
use serde::{de::DeserializeOwned, Serialize};


use crate::{tasks::{BotTask, BotTaskType, events::ActionEvent}, social::source::SocialSource, bots::Bot};

use self::errors::DbError;

pub mod errors;

#[async_trait]
pub trait DbActions
: DeserializeOwned + Unpin + Serialize + Sync + Send {
    type Query: DbQuery + Sync;

    fn get_collection(&self, db: &SocialsDb) -> Collection<Self>;
    fn get_id(&self) -> bson::Uuid;

    async fn insert_db(self: &mut Self, db: &SocialsDb) ->
        Result<mongodb::results::InsertOneResult, crate::db::errors::DbError> {
        let col = self.get_collection(db);
        SocialsDb::insert_one(self, col).await
    }

    async fn update_db(self: &mut Self, db: &SocialsDb) ->
        Result<mongodb::results::UpdateResult, DbError> {
        let id = self.get_id();
        let col = self.get_collection(db);
        SocialsDb::update_by_id(id, self, &col).await
    }

    async fn delete_db(self: &mut Self, db: &SocialsDb) ->
        Result<mongodb::results::DeleteResult, DbError> {
        SocialsDb::delete_by_id(&self.get_id(), &self.get_collection(&db)).await
    }

    async fn find(&self, query: &Self::Query, db: &SocialsDb) -> Result<DbFindResult<Self>, DbError>
    { SocialsDb::find(query, &self.get_collection(db)).await }

    async fn find_by_id(&self, id: bson::Uuid, db: &SocialsDb) -> Result<Option<Self>, DbError> {
        SocialsDb::find_by_id(id, &self.get_collection(db)).await
    }

    async fn get_fresh(self: &mut Self, db: &SocialsDb) -> Result<&mut Self, DbError> {
        match SocialsDb::find_by_id(self.get_id(), &self.get_collection(db))
            .await {
                Ok(r) => match r {
                    Some(t) => { *self = t; Ok(self) }
                    _ => Ok(self)
                },
                Err(e) => Err(e)
            }
    }
}

pub trait DbQuery {
    fn collect_filters(&self) -> Document { Document::new() }
    fn collect_sorting(&self) -> Document { Document::new() }
    fn collect_options(&self) -> FindOptions { FindOptions::default() }
    fn collect_one_options(&self) -> FindOneOptions { FindOneOptions::default() }
}

pub struct DummyQuery;
impl Default for DummyQuery {
    fn default() -> Self { Self }
}
impl DbQuery for DummyQuery {}

#[derive(Debug, Serialize)]
pub struct DbFindResult<T> {
    pub items: Vec<T>,
    pub total: u64
}

#[derive(Clone)]
pub struct SocialsDb {
  pub db_name: String,
  pub client: mongodb::Client,
}

impl SocialsDb {
    pub fn get_db(&self) -> Database { self.client.clone().database(&self.db_name) }
    pub fn collection<T>(&self, name: &str) -> Collection<T> { self.get_db().collection(name) }
    pub fn bots(&self) -> Collection<Bot> { self.get_db().collection("bots") }
    pub fn bots_tasks(&self) -> Collection<BotTask> { self.get_db().collection("bots_tasks") }
    pub fn action_events(&self) -> Collection<ActionEvent> { self.get_db().collection("action_events") }
    pub fn social_sources(&self) -> Collection<SocialSource> { self.get_db().collection("social_sources") }
    pub fn task_types(&self) -> Collection<BotTaskType> { self.get_db().collection("task_types") }

    async fn make_instance (
        connection_env_key: &str,
        db_name_env_key: &str
    ) -> Result<SocialsDb, DbError> {
        // Self::new_test_instance().await
        // parse db connection string
        let connection_string = match env::var(connection_env_key) {
            Ok(c) => c,
            Err(_) => return Err(DbError::db_connection_string())
        };
        // parse db name
        let db_name = match env::var(db_name_env_key) {
            Ok(c) => c,
            Err(_) => return Err(DbError::db_name())
        };
        // parse connection string
        let client_options = match ClientOptions::parse(&connection_string).await {
            Ok(c) => c,
            Err(_) => return Err(DbError::invalid_connection_string())
        };
        // connect db client
        let client = match mongodb::Client::with_options(client_options) {
            Ok(c) => c,
            Err(_) => return Err(DbError::connection_error())
        };
        debug!("Connected to db instance on: {} to {}", &connection_string, db_name);
        let db_client = SocialsDb {
            db_name,
            client
        };
        return Ok(db_client)
    }

    pub async fn new_instance () -> Result<SocialsDb, DbError> {
        return SocialsDb::make_instance(
            "mongo_main_connection",
            "socials_main_db"
        ).await
    }

    pub async fn new_test_instance () -> Result<SocialsDb, DbError> {
        return SocialsDb::make_instance(
            "mongo_test_connection",
            "socials_test_db"
        ).await
    }

    pub async fn find<T, Q>(query: &Q, collection: &Collection<T>) -> Result<DbFindResult<T>, DbError>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
        Q: DbQuery,
    {
        let total = collection.count_documents(query.collect_filters(), None).await.unwrap_or(0);
        let items: Vec<T> = match collection
            .find(query.collect_filters(), query.collect_options())
            .await {
                Ok(cursor) => match cursor.try_collect::<Vec<T>>().await {
                    Ok(items) => items,
                    Err(_) => return Err(DbError::cursor_collect())
                },
                Err(_) => return Err(DbError::error_while_find())
            };

        let res = DbFindResult { items, total };
        Ok(res)
    }

    pub async fn find_one<T, Q>(query: &Q, collection: &Collection<T>) -> Result<Option<T>, DbError>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
        Q: DbQuery,
    {
        match collection
            .find_one(query.collect_filters(), query.collect_one_options()).await {
                Ok(item) => Ok(item),
                Err(_) => Err(DbError::error_while_find())
        }
    }

    pub async fn insert_one<T>(item: impl Borrow<T>, collection: Collection<T>) -> Result<mongodb::results::InsertOneResult, DbError>
    where
        T: Serialize,
    {
        match collection.insert_one(item, None).await  {
            Ok(result) => Ok(result),
            Err(_) => Err(DbError::insert_error(None))
        }
    }

    pub async fn find_by_id<T>(id: bson::Uuid, collection: &Collection<T>) -> Result<Option<T>, DbError>
    where
        T: DeserializeOwned + Unpin + Send + Sync
    {
        let mut f = Document::new();
        f.insert("id", id);
        match collection.find_one(f, None).await {
            Ok(item) => Ok(item),
            Err(_e) => Err(DbError::error_while_find())
        }
    }

    pub async fn update_by_id<T>(id: bson::Uuid, item: impl Borrow<T>, collection: &Collection<T>) -> Result<mongodb::results::UpdateResult, DbError>
    where
        T: Serialize,
    {
        let mut query = Document::new();
        query.insert("id", id);
        match collection.replace_one(query, item, None).await {
            Ok(r) => Ok(r),
            Err(_) => Err(DbError::replace_error(None))
        }
    }

    pub async fn delete_by_id<T>(id: &bson::Uuid, collection: &Collection<T>) -> Result<mongodb::results::DeleteResult, DbError>
    {
        let mut query = Document::new();
        query.insert("id", id);
        match collection.delete_one(query, None).await {
            Ok(r) => Ok(r),
            Err(_) => Err(DbError::delete_error())
        }
    }

    pub async fn delete_many<T, Q>(query: &Q, collection: &Collection<T>) -> Result<mongodb::results::DeleteResult, DbError>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
        Q: DbQuery
    {
        match collection.delete_many(query.collect_filters(), None).await {
            Ok(r) => Ok(r),
            Err(_) => Err(DbError::delete_error())
        }
    }
}
