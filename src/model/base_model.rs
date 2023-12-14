use crate::error::RSparkError;
use crate::model::InnerState;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct BaseModel<'a, T> {
    pub(crate) id: Option<ObjectId>,
    pub(crate) inner: Box<T>,
    pub(crate) inner_state: InnerState,
    pub(crate) db: &'a Database,
    pub(crate) collection_name: &'a str,
    pub(crate) last_error: Option<RSparkError>,
}

impl<'a, T: 'a> Deref for BaseModel<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl<'a, T: 'a> DerefMut for BaseModel<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut()
    }
}

impl<'a, T> BaseModel<'a, T> {
    pub fn __fill(&mut self, inner: T) {
        *self.inner = inner;
    }
    pub fn __set_error(&mut self, error: RSparkError) {
        self.last_error = Some(error);
    }
    pub fn __set_object_id(&mut self, ob_id: Option<ObjectId>) {
        self.id = ob_id;
    }
    pub fn is_filled(&self) -> bool {
        return match self.inner_state {
            InnerState::Filled => true,
            InnerState::Default => false,
        };
    }
    pub fn set_inner_state(&mut self, new_state: InnerState) {
        self.inner_state = new_state;
    }
    pub fn get_inner_state(&self) -> &InnerState {
        &self.inner_state
    }
}

impl<'a, T: Default> BaseModel<'a, T> {
    pub fn new(db: &'a Database, coll_name: &'a str) -> BaseModel<'a, T> {
        BaseModel {
            db,
            inner: Box::new(T::default()),
            inner_state: InnerState::Default,
            id: None,
            collection_name: coll_name,
            last_error: None,
        }
    }
}
