// #[cfg(test)]
// pub mod tests;

use std::{borrow::Borrow, env};

use bson::Document;
use mongodb::{Collection, options::{ClientOptions, FindOptions, FindOneOptions }};
use futures::stream::TryStreamExt;
use serde::{de::DeserializeOwned, Serialize};


use crate::{tasks::BotTask, social::source::SocialSource};

use self::errors::DbError;

pub mod errors;

/*
// TODO fucking async trait / return Future??
struct FindByIdQuery {
    id: bson::Uuid
}

pub trait FindById {
    fn find_by_id<T>(id: bson::Uuid, collection: Collection<T>) -> impl futures::Future<Output = Result<Option<T>, ()>>
    where
        T: DeserializeOwned + Unpin + Send + Sync
    {
        let query = FindByIdQuery {id}; 
        let result = SocialsDb::find_one(query, collection);
        Ok(Some(result))
    }
}
*/

/*
#[async_trait]
pub trait DBActions {
    async fn insert_db<T>(&self, item: T) -> Result<(), ()>
        where
        T: Serialize
    {
        SocialsDb::insert_one(item).await
    }
    fn get_collection<T>(db: &SocialsDb) -> Collection<T>;
    // pub async fn insert_one<T>(item: impl Borrow<T>, collection: Collection<T>) -> Result<(), ()>
}
*/


pub trait DbQuery {
    fn collect_filters(&self) -> Document { Document::new() }
    fn collect_sorting(&self) -> Document { Document::new() }
    fn collect_options(&self) -> FindOptions { FindOptions::default() }
    fn collect_one_options(&self) -> FindOneOptions { FindOneOptions::default() }
}

#[derive(Serialize)]
pub struct DbFindResult<T> {
    pub items: Vec<T>,
    pub total: u32
}

#[derive(Clone)]
pub struct SocialsDb {
  pub db_name: String,
  pub client: mongodb::Client,
}

impl SocialsDb {
    pub fn collection<T>(&self, name: &str) -> Collection<T> {
        self.client.clone().database(&self.db_name).collection(name)
    }
    pub fn bots_tasks(&self) -> Collection<BotTask> {
        self.client.clone().database(&self.db_name).collection("bots_tasks")
    }
    pub fn social_sources(&self) -> Collection<SocialSource> {
        self.client.clone().database(&self.db_name).collection("social_sources")
    }

    pub async fn new_instance () -> Result<SocialsDb, DbError> {
        Self::new_test_instance().await
    }

    pub async fn new_test_instance () -> Result<SocialsDb, DbError> {
        // parse db connection string
        let connection_string = match env::var("mongo_test_connection") {
            Ok(c) => c,
            Err(_) => return Err(DbError::db_connection_string())
        };
        // parse db name
        let db_name = match env::var("socials_test_db") {
            Ok(c) => c,
            Err(_) => return Err(DbError::db_name())
        };
        // parse connection string
        let client_options = match ClientOptions::parse(connection_string).await {
            Ok(c) => c,
            Err(_) => return Err(DbError::invalid_connection_string())
        };
        // connect db client
        let client = match mongodb::Client::with_options(client_options) {
            Ok(c) => c,
            Err(_) => return Err(DbError::connection_error())
        };
        let db_client = SocialsDb {
            db_name,
            client
        };
      return Ok(db_client)
    }

    pub async fn find<T, Q>(query: &Q, collection: &Collection<T>) -> Result<DbFindResult<T>, DbError>
    where
        T: DeserializeOwned + Unpin + Send + Sync,
        Q: DbQuery,
    {
        let items: Vec<T> = match collection
            .find(query.collect_filters(), query.collect_options())
            .await {
                Ok(cursor) => match cursor.try_collect::<Vec<T>>().await {
                    Ok(items) => items,
                    Err(_) => return Err(DbError::cursor_collect())
                },
                Err(_) => return Err(DbError::error_while_find())
            };

        let res = DbFindResult {
            items,
            total: 12
        };
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

    pub async fn insert_one<T>(item: impl Borrow<T>, collection: Collection<T>) -> Result<(), DbError>
    where
        T: Serialize,
    {
        match collection.insert_one(item, None).await  {
            Ok(_) => Ok(()),
            Err(_) => Err(DbError::insert_error(None))
        }
    }

    pub async fn update_by_id<T>(id: bson::Uuid, item: &mut T, collection: &Collection<T>) -> Result<mongodb::results::UpdateResult, DbError>
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
