use mongodb::bson::to_document;
use mongodb::results::InsertOneResult;
use mongodb::Cursor;

use crate::error::RmORMError;
use crate::model::crud::inner_crud::InnerCRUD;
use crate::model::proxy_model::ProxyModel;
use crate::model::utility::inner_utility::InnerUtility;
use crate::model::Prototype::{Doc, Model};
use crate::model::{InnerState, Prototype};
use crate::rm_orm::RmORMResult;
use crate::utilities::convert_to_doc;

#[allow(async_fn_in_trait)]
pub trait ProxyModelCrud<T> {
    async fn save(&mut self) -> RmORMResult<InsertOneResult>;
    async fn update(&mut self) -> RmORMResult<u64>;
    async fn find_one(&mut self, prototype: Prototype<T>) -> RmORMResult<Option<T>>
        where
            T: Send,
            T: Sync,
            T: Unpin
    ;
    async fn find(&self, prototype: Prototype<T>) -> RmORMResult<Cursor<T>>;
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F);

    async fn delete(&self) -> RmORMResult<()>;
}

impl<'a, T> ProxyModelCrud<T> for ProxyModel<'a, T>
    where
        T: InnerCRUD,
        T: Default,
{
    /// insert operation
    async fn save(&mut self) -> RmORMResult<InsertOneResult> {
        T::save(&self.inner, self.db, self.collection_name).await
    }
    /// update operation
    async fn update(&mut self) -> RmORMResult<u64> {
        let doc = convert_to_doc(&self.inner)?;
        let _id = doc.get_object_id("_id");
        println!("the id {:?} ", _id);
        if let Ok(id) = _id {
            return T::update(&id, doc, self.db, self.collection_name).await;
        }
        Err(RmORMError::new("Can't update document without id"))
    }
    async fn find_one(&mut self, prototype: Prototype<T>) -> RmORMResult<Option<T>>
        where
            T: Send,
            T: Sync,
            T: Unpin,
    {
        match prototype {
            Doc(doc) => T::find_one(doc, self.db, self.collection_name).await,
            Model(model) => {
                let converted = convert_to_doc(model);
                match converted {
                    Ok(doc) => T::find_one(doc, self.db, self.collection_name).await,
                    Err(error) => Err(error),
                }
            }
        }

        // match result {
        //     Ok(inner) => {
        //         if let Some(data) = inner {
        //             let id = self.__get_id_from_non_doc(&data);
        //             match id {
        //                 Ok(inner_id) => {
        //                     self.__set_object_id(inner_id);
        //                 }
        //                 Err(error) => {
        //                     self.__set_error(error);
        //                 }
        //             }
        //             self.fill(data);
        //             self.inner_state = InnerState::Filled;
        //         } else {
        //             self.restore_to_default();
        //         }
        //     }
        //     Err(error) => self.__set_error(error),
        // }
        // self
    }
    async fn find(&self, prototype: Prototype<T>) -> RmORMResult<Cursor<T>> {
        match prototype {
            Doc(doc) => T::find(doc, self.db, self.collection_name).await,
            Model(model) => {
                let doc = convert_to_doc(&model)?;
                T::find(doc, self.db, self.collection_name).await
            }
        }
    }
    async fn find_with_callback<F: Fn(T)>(&self, prototype: Prototype<T>, call_back: F) {
        match prototype {
            Doc(doc) => T::find_with_callback(doc, call_back, self.db, self.collection_name).await,
            Model(model) => {
                let converted = to_document(&model);
                if let Ok(doc) = converted {
                    T::find_with_callback(doc, call_back, self.db, self.collection_name).await
                }
            }
        }
    }

    async fn delete(&self) -> RmORMResult<()> {
        let doc = convert_to_doc(&self.inner)?;
        let _id = doc.get_object_id("_id");
        if let Ok(id) = _id{
            T::delete(&id , self.db , self.collection_name).await;
            return Ok(());
        }
        Err(
            RmORMError::new("Id doesn't found ")
        )
    }
}
