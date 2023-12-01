#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::HashMap;
use std::env;
use std::ops::Add;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};
use mongodb::bson::spec::BinarySubtype::UserDefined;
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use serde::{Deserialize, Serialize, Serializer};

use rspark::{Model, RSpark, TModel};
use rspark::utilities::add_coll_name;

#[derive(Model, Serialize, Debug, Deserialize)]
#[coll_name = "Books"]
pub struct Book {
    #[model(unique)]
    name: String,
    author: String,
    the_type: String,
}
#[tokio::test]
async fn main_test() {
    let db = RSpark::connect("admin", "123", "localhost", "27019", "main_db").await;
    let mut my_book = Book::new(
        "Hossein ".to_string(),
        "Salehi".to_string(),
        "My Success".to_string(),
    )
    .await
    .unwrap();
    my_book.save("Book").await;
}
// TODO create a thread test for testing global db in thread

#[tokio::test]
async fn test_all() {
    RSpark::connect("admin", "123", "localhost", "27019", "main_db").await;
    Book::all_with_callback(|book|{
        println!("the Book name is {:?} " , book.name);
    }).await;
}
