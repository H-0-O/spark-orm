use std::borrow::Borrow;

use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{Collection, Cursor, Database};
use mongodb::bson::{doc, Document, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateModifications;
use mongodb::results::InsertOneResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::RSparkError;
use crate::r_spark::{RSpark, RSparkResult};
use crate::utilities::create_index_on_model;

//TODO remove all doc method ; all function of inner_crud should just get an Document as prototype
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
    // Self: Debug,
{
    async fn save(&self, db: &Database, coll_name: &str) -> RSparkResult<InsertOneResult> {
        let collection = Self::get_coll(db, coll_name);
        let re = collection.insert_one(self.borrow(), None).await;
        RSpark::from_mongo_result(re)
    }

    async fn update(&self, object_id: &ObjectId, db: &Database, coll_name: &str) -> RSparkResult<u64> {
        let coll = Self::get_coll(db, coll_name);
        let converted = to_document(self);
        match converted {
            Ok(doc) => {
                let update_doc = UpdateModifications::Document(doc);
                //TODO Turn off upsert for this function
                let result = coll
                    .update_one(
                        doc! {
                            "_id" : object_id
                        },
                        update_doc,
                        None,
                    )
                    .await;
                match result { 
                    Ok(inner_result) => {
                        Ok(inner_result.modified_count)
                    },
                    Err(error) => Err(
                        RSparkError::new(&error.to_string())
                    )
                }
            }
            Err(error) => Err(
                RSparkError::new(
                    &error.to_string()
                )
            )
        }
    }
    async fn find(
        prototype: Document,
        db: &Database,
        coll_name: &str,
    ) -> RSparkResult<Cursor<Self>> {
        let coll = Self::get_coll(db, coll_name);
        let result = coll.find(prototype, None).await;
        RSpark::from_mongo_result(result)
    }
    async fn find_with_callback<F: Fn(Self)>(
        prototype: Document,
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
    async fn find_one(
        prototype: Document,
        db: &Database,
        coll_name: &str,
    ) -> RSparkResult<Option<Self>> {
        let coll = Self::get_coll(db, coll_name);
        RSpark::from_mongo_result(coll.find_one(prototype, None).await)
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

    fn get_coll(db: &Database, coll_name: &str) -> Collection<Self> {
        db.collection::<Self>(coll_name)
    }
}
