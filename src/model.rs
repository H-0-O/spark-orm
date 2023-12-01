use std::borrow::Borrow;

use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{Collection, Cursor};
use mongodb::error::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::RSpark;
use crate::utilities::create_index_on_model;

//TODO write a doc for all methods in here
#[async_trait(?Send)]
pub trait TModel<Struct = Self>
where
    Struct: Serialize,
    Self: DeserializeOwned,
    Self: Borrow<Struct>,
    Self: Sized,
{
    async fn save(&self, collection_name: &str) {
        let db = RSpark::get_db();
        let collection = db.collection::<Struct>(collection_name);
        collection.insert_one(self.borrow(), None).await.unwrap();
    }

    async fn all() -> Result<Cursor<Self>> {
        let coll = Self::get_collection();
        coll.find(None, None).await
    }
    async fn all_with_callback< F: Fn(Self) >(callback: F ){
        let coll = Self::get_collection();
        let stream_curs = coll.find(None , None).await;
        if let Ok(mut docs) = stream_curs {
            while let Some(res_doc) = docs.next().await {
                if let Ok(doc) = res_doc{
                    callback(doc);
                }
            }
        }


    }
    fn get_collection() -> Collection<Self> {
        let db = RSpark::get_db();
        db.collection::<Self>("Book")
    }
    async fn process_attributes(attributes: Vec<String>, collection_name: &str) {
        for attribute in attributes {
            let db = RSpark::get_db();
            let name = format!("__{}__", &attribute);
            let index_model = create_index_on_model(&attribute, &name, true);
            let collection = db.collection::<Struct>(collection_name);
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
}
