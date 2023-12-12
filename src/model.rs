#![allow(dead_code)]


use std::ops::{Deref, DerefMut};
use crate::error::RSparkError;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Database;


pub mod crud;
pub mod inner_crud;
pub mod inner_utility;
pub struct BaseModel<'a, T>
{
    id: Option<ObjectId>,
    inner: Box<Option<T>>,
    db: &'a Database,
    collection_name: &'a str,
    last_error: Option<RSparkError>,
}

impl<'a, T> Deref for BaseModel<'a, T>
where T: 'a
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for BaseModel<'a, T>
where T: 'a
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().as_mut().unwrap()
    }
}


impl<'a, T> BaseModel<'a, T>
{
    pub fn new(db: &'a Database, coll_name: &'a str) -> BaseModel<'a, T> {
        BaseModel {
            db,
            inner: Box::new(None),
            id: None,
            collection_name: coll_name,
            last_error: None,
        }
    }
    pub(crate) fn __fill(&mut self, inner: Option<T>) {
        *self.inner = inner;
    }
    pub(crate) fn __set_error(&mut self, error: RSparkError) {
        self.last_error = Some(error);
    }
    pub(crate) fn __set_object_id(&mut self , ob_id: Option<ObjectId>){
        self.id = ob_id;
    }
    pub fn unwrap_inner(self) -> T {
        self.inner.unwrap()
    }
}



pub enum Prototype<T> {
    Doc(Document),
    Model(T),
}
