#![allow(dead_code , unused_imports , unused_variables)]
use std::env;
use rspark::model::Model;

#[derive(Model)]
struct User {
    f_name: String,
}

#[tokio::test]
async fn main_test() {
    // RSpark::create_global_instance(
    //     "manager_admin",
    //     "admin_root_123",
    //     "localhost",
    //     "27018",
    //     "CRM",
    // )
    //     .await;
    // User::print_name("gfdf");
    println!("Finish Test ");
}


