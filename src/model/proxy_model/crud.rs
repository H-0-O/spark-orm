use async_trait::async_trait;
use mongodb::bson::to_document;
use mongodb::Cursor;

use crate::error::RmORMError;
use crate::model::proxy_model::ProxyModel;
use crate::model::crud::inner_crud::InnerCRUD;
use crate::model::Prototype::{Doc, Model};
use crate::model::Prototype;
use crate::model::utility::inner_utility::InnerUtility;
use crate::rm_orm::RmORMResult;
use crate::utilities::convert_to_doc;

#[async_trait(?Send)]
pub trait ProxyModelCrud<T> {
    async fn save(&mut self) -> &Self;
    async fn update(&mut self) -> &Self;
    async fn find_one(&mut self, prototype: Prototype<T>) -> &Self;
    async fn find(&self, prototype: Prototype<T>) -> RmORMResult<Cursor<T>>;
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F);
}

#[async_trait(?Send)]
impl<'a, T> ProxyModelCrud<T> for ProxyModel<'a, T>
where
    T: InnerCRUD,
    T: Default,
{
    async fn save(&mut self) -> &Self {
        if self.is_filled() {
            let result = T::save(&*self.inner , self.db , self.collection_name ).await;
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
        let converted = convert_to_doc(&self.inner);
        match self._id {
            Some(id) => {
               let result =  match converted {
                    Ok(doc) => T::update(&id , doc , self.db , self.collection_name).await,
                    Err(error) => Err(error)
                };
                if let Err(error) = result{
                    self.__set_error(error);
                }
            },
            None => self.__set_error(RmORMError::new("Can not update without object id "))
        }
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
    async fn find(&self, prototype: Prototype<T>) -> RmORMResult<Cursor<T>> {
        return match prototype {
            Doc(doc) => T::find(doc, self.db, self.collection_name).await,
            Model(model) => {
                let converted = convert_to_doc(&model);
                match converted {
                    Ok(doc) => T::find(doc, self.db, self.collection_name).await,
                    Err(error) => Err(RmORMError::new(&error.to_string())),
                }
            }
        };
    }
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F) {
        match prototype {
            Doc(doc) => {
                T::find_with_callback(doc, call_back, self.db, self.collection_name).await
            }
            Model(model) => {
                let converted = to_document(&model);
                if let Ok(doc) = converted {
                    T::find_with_callback(doc, call_back, self.db, self.collection_name).await
                }
            }
        }
    }
}
