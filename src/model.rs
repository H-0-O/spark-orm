#![allow(dead_code)]

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use getset::Setters;
use mongodb::{Collection, Database};
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneOptions, InsertOneOptions};
use mongodb::results::InsertOneResult;
use serde::de::DeserializeOwned;
use serde::Serialize;
use mongodb::error::Result as MongodbResult;
use crate::Spark;


// impl<M> From<Box<M>> for Document
//     where
//         M: Serialize,
//         M: Sized
// {
//
//     fn from(value: M) -> Self {
//         let result = mongodb::bson::to_document(&value);
//         match result {
//             Ok(doc) => doc,
//             Err(_) => doc! {}
//         }
//     }
// }


#[derive(Debug)]
pub struct Model<'a, M> {
    inner: Box<M>,
    db: Arc<Database>,
    collection_name: &'a str,
    collection: Collection<M>,
}

impl<'a, T: 'a> Deref for Model<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T: 'a> DerefMut for Model<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, M> Model<'a, M>
    where
        M: Default,
        M: Serialize,
        M: DeserializeOwned
{
    /// makes a model and stores the data and collection_name to creating collection object
    /// to store data into it
    ///
    /// # Arguments
    ///
    /// * `db`: you cna pass None , in this way model created by global spark connection , or you can pass your own database
    /// * `collection_name`:  it's collection name that we use in create collection object
    ///
    /// returns: Model<M>
    ///
    /// # Examples
    ///
    /// ```
    /// struct User{
    ///     name: String
    /// }
    /// let db = ...;
    /// let user_model = Model::<User>::new(Arc::clone(db) , "users");
    /// ```
    pub fn new(db: Option<&Arc<Database>>, collection_name: &'a str) -> Model<'a, M> {
        if let Some(database) = db {
            let collection = database.collection::<M>(collection_name);
            return Model {
                inner: Box::new(M::default()),
                db: database.clone(),
                collection_name,
                collection,
            };
        }
        // it panics if it's not initialized before use
        let database = Spark::get_db();
        let collection = database.collection::<M>(collection_name);
        Model {
            inner: Box::new(M::default()),
            db: database,
            collection_name,
            collection,
        }
    }
    pub async fn save(&self, options: impl Into<Option<InsertOneOptions>>)
                      -> MongodbResult<InsertOneResult>
    {
        self.collection.insert_one(
            &*self.inner,
            options,
        ).await
    }
    pub async fn find_one(mut self, doc: impl Into<Document>, options: impl Into<Option<FindOneOptions>>)
                          -> MongodbResult<Option<Self>>
        where
            M: Unpin,
            M: Send,
            M: Sync
    {
        let result = self.collection.find_one(
            Some(doc.into()),
            options,
        ).await?;
        match result {
            Some(inner) => {
                self.fill(inner);
                Ok(
                    Some(self)
                )
            }
            None => Ok(None)
        }
    }

    pub async fn update(&self, doc: impl Into<Document>) {
        let converted = mongodb::bson::to_document(&self.inner);
        let id = converted.unwrap().get("_id").unwrap();
        //TODO complete from here 
        // self.collection.update_one(
        //     doc! {
        //         "_id" : id
        //     },
        //     doc! {
        //         
        //     }
        // );
    }
    pub fn fill(&mut self, inner: M) {
        *self.inner = inner;
    }
}


