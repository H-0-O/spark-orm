use mongodb::{bson::doc, options::IndexOptions, IndexModel};

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