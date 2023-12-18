#![allow(dead_code)]



use mongodb::bson::Document;

pub mod base_model;
pub mod crud;
pub mod utility;
#[derive(Debug)]
pub enum InnerState{
    Filled,
    Default
}


pub enum Prototype<T> {
    Doc(Document),
    Model(T),
}
