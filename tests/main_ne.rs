use std::sync::Arc;
use getset::Setters;
use mongodb::bson;
use mongodb::bson::Document;
use mongodb::Database;
use serde::{Deserialize, Serialize};
// use mongodb::bson::doc;
use spark_orm::Spark;
use spark_orm_derive::Model;

#[Model(coll_name = "users")]
#[derive(Serialize , Deserialize , Default , Debug)]
struct User {
    name: String,
}


//TODO test From trait with struct that has generic
#[Model(coll_name = "users")]
#[derive(Serialize , Deserialize , Default , Debug)]
struct Product {
    name: String,
}

#[derive(Serialize , Debug)]
struct WE{
    name: String
}

impl From<WE> for Document

{
    fn from(value: WE) -> Self {
        mongodb::bson::to_document(&value).unwrap()
    }
}

// impl From<Product> for Document{
//     fn from(value: Product) -> Self {
//         let result = mongodb::bson::to_document(&value);
//         match result {
//             Ok(doc) => doc,
//             Err(_) => doc! {}
//         }
//     }
// }
impl Product {
    pub fn difjo(self){

    }
}
#[tokio::test]
async fn main2() {
    let db = get_db().await;
    let user_model = User::new_model(Some(&db));
    let  mut product_model = Product::new_model(Some(&db));
    product_model.name = "difjodijf".to_string();
    let pr = Product::default();
    product_model.find_one(
        pr,
        None
    ).await.unwrap().unwrap().set_name("difjdofj".to_string());
    // let er = product_model.set_name("difj".to_string());
    println!("the user Model {:?}" , user_model);
}

async fn get_db() -> Arc<Database> {
    Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await
}
