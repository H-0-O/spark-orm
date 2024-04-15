use spark_orm::error::Error;
use spark_orm::model::InnerState;
use spark_orm::Result;
use spark_orm::utilities::convert_to_doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

pub mod crud;

/// A proxy that facilitates interactions between developers and a model.
///
/// This `ProxyModel` struct serves as an intermediary layer, managing various operations
/// between the developer and the underlying model implementation. It encapsulates the
/// model's core functionality, allowing for additional features, error handling, and ...
#[derive(Debug)]
pub struct ProxyModel<'a, T> {
    pub(crate) _id: Option<ObjectId>,
    pub(crate) inner: T,
    pub(crate) inner_state: InnerState,
    pub(crate) db: &'a Database,
    pub(crate) collection_name: &'a str,
    pub(crate) last_error: Option<Error>,
}

/// Implements the `Deref` trait for the `ProxyModel` struct, allowing direct access to the
/// underlying model without the need for the dereference operator (*).
/// The `Deref` trait in Rust enables instances of `ProxyModel` to be treated as if they were
/// instances of the inner model (`T`). This means that developers can access methods and
/// properties of the inner model directly on a `ProxyModel` instance, enhancing code clarity
/// and reducing the need for explicit dereferencing with the `*` operator.
impl<'a, T: 'a> Deref for ProxyModel<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T: 'a> DerefMut for ProxyModel<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl<'a, T> ProxyModel<'a, T> {
    pub(crate) fn __set_error(&mut self, error: Error) {
        self.last_error = Some(error);
    }
    pub(crate) fn __set_object_id(&mut self, ob_id: Option<ObjectId>) {
        self._id = ob_id;
    }
    pub(crate) fn __get_id_from_non_doc<M: Serialize>(
        &self,
        data: &M,
    ) -> Result<Option<ObjectId>> {
        let converted = convert_to_doc(data);

        match converted {
            Ok(doc) => {
                let id = doc.get("_id");
                if let Some(bson_id) = id {
                    return Ok(bson_id.as_object_id());
                }
                Err(Error::new(""))
            }
            Err(error) => Err(error),
        }
    }
    pub fn is_filled(&self) -> bool {
        match self.inner_state {
            InnerState::Filled => true,
            InnerState::Default => false,
        }
    }
    pub fn set_inner_state(&mut self, new_state: InnerState) -> &Self {
        self.inner_state = new_state;
        self
    }
    pub fn get_inner_state(&self) -> &InnerState {
        &self.inner_state
    }
    pub fn get_last_error(&self) -> Option<&Error> {
        self.last_error.as_ref()
    }
    pub fn has_error(&self) -> bool {
        self.last_error.is_some()
    }
    pub fn set_id(&mut self , id: Option<ObjectId>){
        self.__set_object_id(id);
    }
}

impl<'a, T: Default> ProxyModel<'a, T> {
    pub fn new(db: &'a Database, coll_name: &'a str) -> ProxyModel<'a, T> {
        ProxyModel {
            db,
            inner: T::default(),
            inner_state: InnerState::Default,
            _id: None,
            collection_name: coll_name,
            last_error: None,
        }
    }
}
