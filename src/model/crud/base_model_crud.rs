use async_trait::async_trait;
use mongodb::bson::to_document;
use mongodb::Cursor;

use crate::error::RSparkError;
use crate::model::{InnerState, Prototype};
use crate::model::base_model::BaseModel;
use crate::model::crud::inner_crud::InnerCRUD;
use crate::model::Prototype::{Doc, Model};
use crate::r_spark::RSparkResult;
use crate::utilities::convert_to_doc;

#[async_trait(?Send)]
pub trait BaseModelCrud<T> {
    async fn save(&mut self) -> &Self;
    async fn update(&mut self) -> &Self;
    async fn find_one(&mut self, prototype: Prototype<T>) -> &Self;
    async fn find(&self, prototype: Prototype<T>) -> RSparkResult<Cursor<T>>;
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F);
    // fn set_object_id(&mut self, object_id: Option<ObjectId>);
}

#[async_trait(?Send)]
impl<'a, T> BaseModelCrud<T> for BaseModel<'a, T>
where
    T: InnerCRUD,
{
    async fn save(&mut self) -> &Self {
        if self.is_filled(){
            let result = self.inner.save(self.db , self.collection_name).await;
            match result {
                Ok(inner_result) => {

                },
                Err(error) => {
                    self.__set_error(error);
                    self.set_inner_state(InnerState::Default);
                }
            }
        }
        // if let Some(inner) = self.inner.deref() {
        //     let operation_result = inner.save(self.db, self.collection_name).await;
        //     match operation_result {
        //         Ok(inner_re) => {
        //             let ob_id = inner_re.inserted_id.as_object_id();
        //             self.__set_object_id(ob_id);
        //         }
        //         Err(error) => self.__set_error(error),
        //     };
        // } else {
        //     self.__set_error(RSparkError::new("Can not save empty document"));
        // }
        self
    }
    async fn update(&mut self) -> &Self {
        // match &*self.inner {
        //     Some(inner) => {
        //         if let Some(id) = self.id {
        //             let update_result = inner.update(&id, self.db, self.collection_name).await;
        //             // TODO decide about it later
        //         }
        //         self.__set_error(RSparkError::new("Can not update the doc"));
        //     }
        //     None => self.__set_error(RSparkError::new("Can not update empty doc")),
        // };
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
        // match result {
        //     Ok(funded) => {
        //         self.__fill(funded);
        //     }
        //     Err(error) => self.__set_error(error),
        // }
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
    // fn set_object_id(&mut self, object_id: Option<ObjectId>) {
    //     self.id = object_id;
    // }
}
