use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use rspark::model::InnerState;
use rspark::{model::Prototype, RSpark};
use rspark_derive::Model;
#[derive(Model, Serialize, Deserialize, Debug, Default)]
#[coll_name = "Books"]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    _id: Option<ObjectId>,
    #[model(unique)]
    name: String,
    other_info: OtherInfo,
    some: u8,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OtherInfo {
    print_number: u32,
    fee: u8,
}
#[derive(Serialize)]
pub struct RawTest {
    age: u8,
    name: &'static str,
}
// TODO move the tests into a tests/crud.rs file
// TODO
#[tokio::test]
async fn _save() {
    let db = get_test_db().await;
    let mut new_book = Book::new(&db).await;
    new_book.set_inner_state(InnerState::Filled);
    new_book.name = "The First Book".to_string();
    new_book.other_info.print_number = 56;
    new_book.some = 23;
    new_book.other_info.fee = 5;
    new_book.save().await;
    if new_book.has_error() {
        let error = new_book.get_last_error().unwrap();
        println!("my error {:?}", error)
    }
}

#[tokio::test]
async fn __raw_save() {
    let db = get_test_db().await;
    let test = RawTest {
        name: "Hossein",
        age: 78,
    };
    let re = db
        .collection::<RawTest>("Books")
        .insert_one(test, None)
        .await;
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
    let prototype: Prototype<Book> = Prototype::Model(Book {
        _id: None,
        name: "The First Book".to_string(),
        other_info: OtherInfo {
            fee: 5,
            print_number: 56,
        },
        some: 23,
    });
    let last = the_book.find_one(prototype).await;
}

#[tokio::test]
async fn __update() {
    let db = get_test_db().await;
    let mut the_book = Book::new(&db).await;
}
fn get_test_book() -> Book {
    Book {
        _id: None,
        some: 52,
        name: "Hassan".to_string(),
        other_info: OtherInfo::default(),
    }
}

async fn get_test_db() -> Database {
    RSpark::connect("admin", "123", "localhost", "27019", "main_db").await
}
