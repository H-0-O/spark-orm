use std::borrow::Borrow;
use std::fmt::Debug;

use async_trait::async_trait;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::results::InsertOneResult;
use mongodb::{Collection, Cursor, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::RmORMError;
use crate::rm_orm::{RmORM, RmORMResult};


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
    async fn save(inner: &Self, db: &Database, coll_name: &str) -> RmORMResult<InsertOneResult> {
        let collection = Self::get_coll(db, coll_name);
        let re = collection.insert_one(inner, None).await;
        RmORM::from_mongo_result(re)
    }

    async fn update(
        object_id: &ObjectId,
        inner: Document,
        db: &Database,
        coll_name: &str,
    ) -> RmORMResult<u64> {
        let coll = Self::get_coll(db, coll_name);
        let result = coll
            .update_one(
                doc! {
                    "_id" : object_id
                },
                inner,
                None,
            )
            .await;
        match result {
            Ok(inner_result) => Ok(inner_result.modified_count),
            Err(error) => Err(RmORMError::new(&error.to_string())),
        }
    }
    async fn find(
        prototype: Document,
        db: &Database,
        coll_name: &str,
    ) -> RmORMResult<Cursor<Self>> {
        let coll = Self::get_coll(db, coll_name);
        let result = coll.find(prototype, None).await;
        RmORM::from_mongo_result(result)
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
    ) -> RmORMResult<Option<Self>> {
        let coll = Self::get_coll(db, coll_name);
        RmORM::from_mongo_result(coll.find_one(prototype, None).await)
    }
    async fn process_attributes(attributes: Vec<String>, collection_name: &str) {
        todo!();
        // for attribute in attributes {
        //     let db = RSpark::get_db();
        //     let name = format!("__{}__", &attribute);
        //     let index_model = create_index_on_model(&attribute, &name, true);
        //     let collection = db.collection::<Self>(collection_name);
        //     match collection.create_index(index_model, None).await {
        //         Ok(_) => {
        //             panic!("this is register")
        //         }
        //         Err(error) => panic!(
        //             "can not create the index because :  {:?} ",
        //             error.to_string()
        //         ),
        //     }
        // }
    }

    fn get_coll(db: &Database, coll_name: &str) -> Collection<Self> {
        db.collection::<Self>(coll_name)
    }
}
