use async_trait::async_trait;
use mongodb::bson::to_document;
use mongodb::Cursor;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::error::RSparkError;
use crate::model::base_model::BaseModel;
use crate::model::crud::inner_crud::InnerCRUD;
use crate::model::Prototype::{Doc, Model};
use crate::model::Prototype;
use crate::model::utility::inner_utility::InnerUtility;
use crate::r_spark::RSparkResult;
use crate::utilities::convert_to_doc;

#[async_trait(?Send)]
pub trait BaseModelCrud<T> {
    async fn save(&mut self) -> &Self;
    async fn update(&mut self) -> &Self;
    async fn find_one(&mut self, prototype: Prototype<T>) -> &Self;
    async fn find(&self, prototype: Prototype<T>) -> RSparkResult<Cursor<T>>;
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F);
}

#[async_trait(?Send)]
impl<'a, T> BaseModelCrud<T> for BaseModel<'a, T>
where
    T: InnerCRUD,
    T: Default,
{
    async fn save(&mut self) -> &Self {
        if self.is_filled() {
            let doc = convert_to_doc(&self.inner);
            let result = match doc {
                Ok(doc) => {
                    T::save(doc , self.db , self.collection_name).await
                },
                Err(error) => Err(error)
            };
            match result {
                Ok(inner_result) => {
                    let object_id = inner_result.inserted_id.as_object_id();
                    self.__set_object_id(object_id);
                }
                Err(error) => {
                    self.__set_error(error);
                    self.restore_to_default();
                }
            }
        }
        self
    }
    async fn update(&mut self) -> &Self {
        self
    }
    async fn find_one(&mut self, prototype: Prototype<T>) -> &Self {
        let result = match prototype {
            Doc(doc) => T::find_one(doc, self.db, self.collection_name).await,
            Model(model) => {
                let converted = convert_to_doc(model);
                match converted {
                    Ok(doc) => T::find_one(doc, self.db, self.collection_name).await,
                    Err(error) => Err(error),
                }
            }
        };
        match result {
            Ok(inner) => self.set_or_default(inner),
            Err(error) => self.__set_error(error),
        }
        self
    }
    async fn find(&self, prototype: Prototype<T>) -> RSparkResult<Cursor<T>> {
        return match prototype {
            Doc(doc) => T::find(doc, self.db, self.collection_name).await,
            Model(model) => {
                let converted = to_document(&model);
                match converted {
                    Ok(doc) => T::find(doc, self.db, self.collection_name).await,
                    Err(error) => Err(RSparkError::new(&error.to_string())),
                }
            }
        };
    }
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F) {
        match prototype {
            Prototype::Doc(doc) => {
                T::find_with_callback(doc, call_back, self.db, self.collection_name).await
            }
            Prototype::Model(model) => {
                let converted = to_document(&model);
                if let Ok(doc) = converted {
                    T::find_with_callback(doc, call_back, self.db, self.collection_name).await
                }
            }
        }
    }
}
