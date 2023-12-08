use std::borrow::Borrow;
use std::fmt::Debug;

use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use mongodb::{Collection, Cursor, Database};
use mongodb::bson::{Document, to_document};
use mongodb::results::InsertOneResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::RSparkError;
use crate::r_spark::{RSpark, RSparkResult};
use crate::utilities::create_index_on_model;

#[async_trait(?Send)]
pub trait InnerCRUD
where
    Self: Serialize,
    Self: DeserializeOwned,
    Self: Borrow<Self>,
    Self: Sized,
    Self: Unpin,
    Self: Sync,
    Self: Send,
    Self: Debug,
{
    async fn save(&self, db: &Database, coll_name: &str) -> RSparkResult<InsertOneResult> {
        let collection = db.collection::<Self>(coll_name);
        let re = collection.insert_one(self.borrow(), None).await;
        RSpark::from_mongo_result(re)
    }
    async fn find(
        prototype: Option<Self>,
        db: &Database,
        coll_name: &str,
    ) -> RSparkResult<Cursor<Self>> {
        let coll = Self::get_coll(db, coll_name);
        let converted = to_document(&prototype);
        match converted {
            Ok(doc) => RSpark::from_mongo_result(coll.find(doc, None).await),
            Err(error) => Err(RSparkError::new(&error.to_string())),
        }
    }
    async fn find_with_callback<F: Fn(Self)>(
        prototype: Option<Self>,
        callback: F,
        db: &Database,
        coll_name: &str,
    ) {
        let stream_curs = Self::find(prototype, db, coll_name).await;
        if let Ok(mut docs) = stream_curs {
            while let Some(res_doc) = docs.next().await {
                if let Ok(doc) = res_doc {
                    callback(doc);
                }
            }
        }
    }
    async fn process_attributes(attributes: Vec<String>, collection_name: &str) {
        todo!();
        for attribute in attributes {
            let db = RSpark::get_db();
            let name = format!("__{}__", &attribute);
            let index_model = create_index_on_model(&attribute, &name, true);
            let collection = db.collection::<Self>(collection_name);
            match collection.create_index(index_model, None).await {
                Ok(_) => {
                    panic!("this is register")
                }
                Err(error) => panic!(
                    "can not create the index because :  {:?} ",
                    error.to_string()
                ),
            }
        }
    }
    async fn __find(prototype: Self, db: &Database, coll_name: &str) -> RSparkResult<Option<Self>> {
        let coll = Self::get_coll(db, coll_name);
        let converted = to_document(&prototype);

        return match converted {
            Ok(doc) => {
                return match coll.find(doc, None).await {
                    Ok(mut funded) => {
                        return if let Ok(result) = funded.try_next().await {
                            Ok(result)
                        } else {
                            Ok(None)
                        }
                    }
                    Err(error) => Err(RSparkError::new(&error.to_string())),
                };
            }
            Err(error) => Err(RSparkError::new(&error.to_string())),
        };
    }
    async fn find_one_with_doc(
        prototype: Document,
        db: &Database,
        coll_name: &str,
    ) -> RSparkResult<Option<Self>> {
        let coll = Self::get_coll(db, coll_name);
        return match coll.find_one(prototype, None).await {
            Ok(funded) => Ok(funded),
            Err(error) => Err(RSparkError::new(&error.to_string())),
        };
    }
    async fn find_one(
        prototype: Self,
        db: &Database,
        coll_name: &str,
    ) -> RSparkResult<Option<Self>> {
        let converted = to_document(&prototype);
        return match converted {
            Ok(doc) => Self::find_one_with_doc(doc, db, coll_name).await,
            Err(error) => Err(RSparkError::new(&error.to_string())),
        };
    }
    fn get_coll(db: &Database, coll_name: &str) -> Collection<Self> {
        db.collection::<Self>(coll_name)
    }
}
