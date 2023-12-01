use std::collections::HashMap;
use std::sync::Mutex;
use mongodb::{bson::doc, IndexModel, options::IndexOptions};
use once_cell::sync::Lazy;
static COLLECTION_NAMES: Lazy<Mutex<HashMap<String , String>>> = Lazy::new(||{
   Mutex::new( HashMap::new() )
});

pub fn add_coll_name(model_name: String , col_name: String){
    let global = COLLECTION_NAMES.lock();
    if let Ok(mut mug) = global {
        mug.insert(model_name , col_name);
    } 
}


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