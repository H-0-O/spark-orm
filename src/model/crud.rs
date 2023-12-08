use std::ops::Deref;

use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;
use mongodb::Cursor;

use crate::{error::RSparkError, model::BaseModel};
use crate::model::inner_crud::InnerCRUD;
use crate::r_spark::RSparkResult;

#[async_trait(?Send)]
pub trait BaseModelCrud<'a, T> {
    async fn save(&mut self) -> RSparkResult<ObjectId>;
    async fn find_one(&self, prototype: T) -> RSparkResult<Option<T>>;
    async fn find_one_with_doc(&self, prototype: Document) -> RSparkResult<Option<T>>;
    async fn find<F: Fn(T)>(&self , prototype : Option<T> , call_back: Option<F>) -> RSparkResult<Option<Cursor<T>>>;
    fn set_object_id(&mut self, object_id: Option<ObjectId>);
}

#[async_trait(?Send)]
impl<'a, T> BaseModelCrud<'a, T> for BaseModel<'a, T>
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
    async fn find_one(&self, prototype: T) -> RSparkResult<Option<T>> {
        T::find_one(prototype, self.db, self.collection_name).await
    }
    async fn find_one_with_doc(&self, prototype: Document) -> RSparkResult<Option<T>> {
        T::find_one_with_doc(prototype, self.db, self.collection_name).await
    }
    async fn find<F: Fn(T)>(&self , prototype: Option<T> , callback: Option<F>) -> RSparkResult<Option<Cursor<T>>> {
        return if let Some(fn_call) = callback {
            T::find_with_callback(prototype , fn_call , self.db , self.collection_name).await;
            Ok(None)
        } else{
            return match T::find(prototype , self.db , self.collection_name).await {
                Ok(result) => {
                    Ok(Some(result))
                },
                Err(error) => {
                    Err(
                        error
                    )
                }
            }
        };
    }
    // fn get_object_id(&self) -> Option<ObjectId> {
    //     self.id
    // }
    fn set_object_id(&mut self, object_id: Option<ObjectId>) {
        self.id = object_id;
    }
}
