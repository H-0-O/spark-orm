#![allow(dead_code, unused_imports, unused_variables)]

use std::collections::HashMap;
use std::env;
use std::ops::Add;
use std::sync::Mutex;

use mongodb::{Client, Database};
use mongodb::bson::doc;
use mongodb::bson::spec::BinarySubtype::UserDefined;
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize, Serializer};

use rspark::{Model, RSpark , TModel};

#[derive(Model, Serialize, Debug)]
pub struct Book {
    #[model(unique)]
    name: String,
    #[model(unique)]
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

pub async fn create_client_options(
    user_name: &str,
    password: &str,
    host: &str,
    port: &str,
) -> ClientOptions {
    let connection_string = format!("mongodb://{}:{}@{}:{}", user_name, password, host, port);
    let client_options = ClientOptions::parse(connection_string).await;
    match client_options {
        Ok(otp) => otp,
        Err(err) => panic!("P"),
    }
}
