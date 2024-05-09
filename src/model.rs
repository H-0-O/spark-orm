#![allow(dead_code)]

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::process::Output;
use std::sync::Arc;
use mongodb::{Collection, Cursor, Database};
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneOptions, FindOptions, InsertOneOptions, UpdateOptions};
use serde::de::DeserializeOwned;
use serde::{Serialize};
use mongodb::error::Result as MongodbResult;
use mongodb::results::UpdateResult;
use crate::futures::StreamExt;
use crate::Spark;

// TODO: this must move to types module
type Id = mongodb::bson::Bson;

#[derive(Debug, Serialize)]
pub struct Model<'a, M>
{
    inner: Box<M>,
    #[serde(skip)]
    db: Arc<Database>,
    #[serde(skip)]
    collection_name: &'a str,
    #[serde(skip)]
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
        M: DeserializeOwned,
        M: Send,
        M: Sync,
        M: Unpin,
        M: Debug
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
                inner: Box::<M>::default(),
                db: database.clone(),
                collection_name,
                collection,
            };
        }
        // it panics if it's not initialized before use
        let database = Spark::get_db();
        let collection = database.collection::<M>(collection_name);
        Model {
            inner: Box::<M>::default(),
            db: database,
            collection_name,
            collection,
        }
    }

    /// saves the change , if the inner has some _id then it's update the existing unless
    /// it's create  new document 
    pub async fn save(&mut self, options: impl Into<Option<InsertOneOptions>>)
                      -> MongodbResult<Id>
    {
        let mut converted = mongodb::bson::to_document(&self.inner)?;
        if let Some(id) = converted.get("_id") {
            let owned_id = id.to_owned();
            let upsert = self.collection.update_one(
                doc! {
                    "_id" : id
                },
                doc! { "$set": converted},
                Some(
                    UpdateOptions::builder().upsert(Some(true)).build()
                ),
            ).await?;
            return if upsert.upserted_id.is_some() {
                Ok(upsert.upserted_id.unwrap())
            } else {
                Ok(owned_id)
            };
        } else {
            converted.remove("_id");
        }

        let re = self.collection.insert_one(
            &*self.inner,
            options,
        ).await?;
        Ok(re.inserted_id)
    }
    pub async fn find_one(mut self, doc: impl Into<Document>, options: impl Into<Option<FindOneOptions>>)
                          -> MongodbResult<Option<Self>>
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

    /// this is raw update , and you can pass document or your model 
    /// # Examples
    /// ## with the raw doc
    ///  ```
    ///  let user_model = User::new_model(Some(&db));
    ///     let updated = user_model.update(
    ///         doc! {
    ///             "name": "Hossein",
    ///         },
    ///         doc! {
    ///            "$set": {
    ///                 "name": "Hossein 33"
    ///             }
    ///         },
    ///         None,
    ///     ).await.unwrap(); 
    /// ```
    /// ## with the model 
    /// let user_model = User::new_model(Some(&db));
    ///     let mut sample_user = User::default();
    ///     sample_user.name = "Hossein 33".to_string();
    ///     let updated = user_model.update(
    ///         &sample_user,
    ///        doc! {
    ///            "$set": {
    ///                "name": "Hossein 3355"
    ///            }
    ///        },
    ///        None,
    ///    ).await.unwrap();
    ///
    /// ## with_model_instance 
    ///     let mut user_model = User::new_model(Some(&db));
    ///    user_model.name = "Hossein 3355".to_string();
    ///    user_model.age = 58;
    ///    let updated = user_model.update(
    ///        &user_model,
    ///        doc! {
    ///            "$set": {
    ///                "name": "Hossein 325"
    ///            }
    ///        },
    ///        None,
    ///    ).await.unwrap();
    ///
    ///
    ///
    pub async fn update(&self, query: impl Into<Document>, doc: impl Into<Document>, options: impl Into<Option<UpdateOptions>>)
                        -> MongodbResult<UpdateResult>
    {
        self.collection.update_one(
            query.into(),
            doc.into(),
            options,
        ).await
    }

    pub async fn find(&self, filter: impl Into<Option<Document>>, options: impl Into<Option<FindOptions>>)
                      -> MongodbResult<Cursor<M>>
    {
        self.collection.find(
            filter,
            options,
        ).await
    }

    pub async fn find_and_collect(&self, filter: impl Into<Option<Document>>, options: impl Into<Option<FindOptions>>)
                               -> MongodbResult<Vec<MongodbResult<M>>>
    {
        let future = self.collection.find(
            filter,
            options,
        ).await?;
        Ok(future.collect().await)
    }
    pub fn fill(&mut self, inner: M) {
        *self.inner = inner;
    }
}

// converts

impl<'a, M> From<Model<'a, M>> for Document
    where
        M: Serialize,
{
    fn from(value: Model<M>) -> Self {
        mongodb::bson::to_document(&value.inner).unwrap()
    }
}

impl<'a, M> From<&Model<'a, M>> for Document
    where
        M: Serialize
{
    fn from(value: &Model<'a, M>) -> Self {
        mongodb::bson::to_document(&value.inner).unwrap()
    }
}
