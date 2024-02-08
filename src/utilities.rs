use mongodb::bson::{to_document, Document};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};
use serde::Serialize;

use crate::error::RmORMError;
use crate::rm_orm::RmORMResult;

pub fn create_index_on_model(field_name: &str, name: &str, unique: bool) -> IndexModel {
    let index_options = IndexOptions::builder()
        .unique(Some(unique))
        .name(Some(name.to_string()))
        .build();
    IndexModel::builder()
        .keys(doc! {
            field_name: 1
        })
        .options(index_options)
        .build()
}

pub(crate) fn convert_to_doc<T: Serialize>(model: T) -> RmORMResult<Document> {
    let converted = to_document(&model);
    match converted {
        Ok(doc) => {
            Ok(doc)
        }
        Err(error) => Err(RmORMError::new(&error.to_string())),
    }
}
