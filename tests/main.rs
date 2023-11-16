#![allow(dead_code , unused_imports , unused_variables )]
use std::env;
use std::sync::Mutex;
use rspark::model::Model;
use rspark::RSpark;
use serde::{Deserialize, Serialize, Serializer};

#[derive( Model , Serialize , Deserialize)]
pub struct User {
    f_name: String,
    l_name : Option<String>,
}
impl ModelUser for User{
    fn new(f_name: String, l_name: Option<String>) -> Self {
        todo!()
    }
}

#[tokio::test]
async fn main_test() {
    RSpark::create_global(
        "manager_admin",
            "admin_root_123",
            "localhost",
            "27018",
            "CRM",
    ).await;
    let db = RSpark::get_global_db();
    let f = db.collection::<User>("User");
    // let the_user = User::new("Hossein".to_string() , None);
    // db.collection("User").insert_one(User {
    //     f_name: "".to_string(),
    //     l_name: None,
    // }, None).await.expect("TODO: panic message");
    // User::print_name("gfdf");
    println!("Finish Test ");

}

// TODO create a thread test for testing global db in thread


