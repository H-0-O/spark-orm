use async_trait::async_trait;
use mongodb::Database;
use serde::Serialize;
use std::borrow::Borrow;
use crate::RSpark;
use crate::utilities::create_index_on_model;

#[async_trait(?Send)]
pub trait TModel<Struct = Self>
    where
        Struct: Serialize,
        Self: Borrow<Struct>,
        Self: Sized,
{
    async fn save(&self, collection_name: &str) {
        let db = RSpark::get_db();
        let collection = db.collection::<Struct>(collection_name);
        collection.insert_one(self.borrow(), None).await.unwrap();
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
