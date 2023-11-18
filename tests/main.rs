#![allow(dead_code, unused_imports, unused_variables)]

use mongodb::bson::spec::BinarySubtype::UserDefined;

use serde::{Deserialize, Serialize, Serializer};
use std::env;
use std::ops::Add;

use std::sync::Mutex;

use rspark::Model;
use rspark::RSpark;

// #[derive( Model , Serialize , Deserialize)]
// pub struct User<'a> {
//     #[serde(skip_serializing)]
//     f_name: &'a str ,
// }

#[derive(Model)]
pub struct Book {
    #[model(unique)]
    name: String,
    #[model(unique)]
    l_name: String,
}

#[derive(Model)]
pub struct User {
    #[model(unique)]
    user_name: String,
    age: u16,
}

#[tokio::test]
async fn main_test() {
    // let db = RSpark::create_global(
    //     "manager_admin",
    //     "admin_root_123",
    //     "localhost",
    //     "27018",
    //     "CRM",
    // )
    // .await;

    let db = RSpark::new("dfdf", "dfdf", "localhost", "2020", "dfdf")
        .await
        .unwrap();

    Book::new("hoosein".to_string(), "hassan".to_string(), &db.get_db()).await;
    Book::new("hoosein".to_string(), "hassan".to_string(), &db.get_db()).await;
    User::new("Hossein".to_string(), 22, &db.get_db()).await;

    println!("every thing is normal");
    // let f = db.collection::<User>("User");
    // let the_user = User::new("Hossein".to_string() , None);
    // db.collection("User").insert_one(User {
    //     f_name: "".to_string(),
    //     l_name: None,
    // }, None).await.expect("TODO: panic message");
    // User::print_name("gfdf");
    println!("Finish Test ");
}

// TODO create a thread test for testing global db in thread
