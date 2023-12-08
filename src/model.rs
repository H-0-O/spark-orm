#![allow(dead_code)]

use mongodb::bson::oid::ObjectId;
use mongodb::Database;

pub mod crud;
pub mod inner_crud;

pub struct BaseModel<'a, T> {
    pub id: Option<ObjectId>,
    pub inner: Box<Option<T>>,
    pub db: &'a Database,
    pub collection_name: &'a str,
}