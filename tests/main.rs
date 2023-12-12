use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use rspark::model::inner_utility::InnerUtility;
use rspark::model::Prototype;
use rspark::{
    model::{crud::BaseModelCrud, inner_crud::InnerCRUD, BaseModel},
    Model, RSpark,
};

#[derive(Model, Serialize, Deserialize , Debug)]
#[coll_name = "Books"]
pub struct Book {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
    #[model(unique)]
    name: String,
}
// TODO move the tests into a tests/crud.rs file
// TODO
#[tokio::test]
async fn _save() {
    let db = get_test_db().await;
    let mut the_book = Book::new(&db).await;
    the_book.fill(get_test_book());
    the_book.save().await;
}

#[tokio::test]
async fn __find() {
    let db = get_test_db().await;
    let the_book = Book::new(&db).await;
    let result = the_book.find(Prototype::Model(get_test_book())).await;
    match result {
        Ok(mut stream) => {
            while let Some(res_doc) = stream.next().await {
                if let Ok(book) = res_doc {
                    // println!(" The Book is {:?} ", book);
                }
            }
        }
        Err(error) => {
            panic!(" {} ", error.to_string())
        }
    }

    println!("__find is passed ");
}

#[tokio::test]
async fn __find_one_with_doc() {
    let db = get_test_db().await;
    let mut the_book = Book::new(&db).await;
    let prototype = Prototype::Doc(doc! {
            "name": "SomeThing"
    });
    the_book.find_one(prototype).await;
}

#[tokio::test]
async fn __find_one_with_model() {
    let db = get_test_db().await;
    let mut the_book = Book::new(&db).await;
    let prototype = Prototype::Model(get_test_book());
    let last =
        the_book.find_one(prototype).await;


    // println!("result {:?} ", re);
}

#[tokio::test]
async fn __update() {
    let db = get_test_db().await;
    let mut the_book = Book::new(&db).await;
     the_book.find_one(Prototype::Model(get_test_book())).await;
    the_book.name = "df".to_string();
}
fn get_test_book() -> Book {
    Book {
        _id: None,
        name: "Hassan".to_string(),
    }
}

async fn get_test_db() -> Database {
    RSpark::connect("admin", "123", "localhost", "27019", "main_db").await
}
