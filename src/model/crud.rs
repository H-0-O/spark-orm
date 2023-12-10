use std::ops::Deref;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::to_document;
use mongodb::Cursor;

use crate::model::inner_crud::InnerCRUD;
use crate::model::Prototype;
use crate::model::Prototype::{Doc, Model};
use crate::r_spark::RSparkResult;
use crate::{error::RSparkError, model::BaseModel};

#[async_trait(?Send)]
pub trait BaseModelCrud<T> {
    async fn save(&mut self) -> RSparkResult<ObjectId>;
    async fn update(&self) -> RSparkResult<u64>;
    async fn find_one(&self, prototype: Prototype<T>) -> RSparkResult<Option<T>>;
    async fn find<F: Fn(T)>(&self, prototype: Prototype<T>) -> RSparkResult<Cursor<T>>;
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F);
    fn set_object_id(&mut self, object_id: Option<ObjectId>);
}

#[async_trait(?Send)]
impl<'a, T> BaseModelCrud<T> for BaseModel<'a, T>
where
    T: InnerCRUD,
{
    async fn save(&mut self) -> RSparkResult<ObjectId> {
        if let Some(inner) = self.inner.deref() {
            let operation_result = inner.save(self.db, self.collection_name).await;
            return match operation_result {
                Ok(inner_re) => {
                    let ob_id = inner_re.inserted_id.as_object_id();
                    if let Some(inner_ob_id) = ob_id {
                        return Ok(inner_ob_id);
                    }
                    Err(RSparkError::new("The object id "))
                }
                Err(error) => Err(error),
            };
        }
        Err(RSparkError::new("Can not save empty document"))
    }

    async fn update(&self) -> RSparkResult<u64> {

        return match *self.inner {
            Some(inner) => {
                if let Some(id) = self.id {
                    return inner.update(&id, self.db, self.collection_name).await;
                }
                Err(RSparkError::new("Can not update the doc"))
            }
            None => return Err(RSparkError::new("Can not update empty doc")),
        };

    }
    async fn find_one(&self, prototype: Prototype<T>) -> RSparkResult<Option<T>> {
        match prototype {
            Doc(doc) => T::find_one(doc, self.db, self.collection_name).await,
            Model(model) => {
                let converted = to_document(&model);
                return match converted {
                    Ok(doc) => T::find_one(doc, self.db, self.collection_name).await,
                    Err(error) => Err(RSparkError::new(&error.to_string())),
                };
            }
        }
    }
    async fn find<F: Fn(T)>(&self, prototype: Prototype<T>) -> RSparkResult<Cursor<T>> {
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
    fn set_object_id(&mut self, object_id: Option<ObjectId>) {
        self.id = object_id;
    }
}
