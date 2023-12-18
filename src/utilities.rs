use mongodb::{bson::doc, IndexModel, options::IndexOptions};
use mongodb::bson::{Document, to_document};
use serde::Serialize;


use crate::error::RmORMError;
use crate::RSparkResult;

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

pub(crate) fn convert_to_doc<T:Serialize>(model: T) -> RSparkResult<Document> {
    let converted = to_document(&model);
    return match converted {
        Ok(mut doc) => {
            doc.remove("_id");
            Ok(doc)
        },
        Err(error) => {
            Err(
                RmORMError::new(&error.to_string())
            )
        }
    };
}

