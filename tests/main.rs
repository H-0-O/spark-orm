use std::sync::Arc;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use spark_orm::{Model, ProxyModelCrud, Spark};
use spark_orm::model::crud::inner_crud::InnerCRUD;
use spark_orm::model::Prototype;

#[Model(coll_name = "users")]
#[derive(Serialize, Deserialize, Default, Debug)]
struct User {
    age: u32,
    name: String,
    email: String,
}


#[tokio::test]
async fn create() {
    let db = get_db().await;
    let mut user = User::new_model(&db);
    user.name = "Hossein".to_string();
    user.email = "spark_orm_test".to_string();
    let re = user.save().await;

    // This unwraps the result and return insert ID
    re.unwrap();
}


#[tokio::test]
async fn find_one() {
    let db = get_db().await;
    let mut user = User::new_model(&db);
    let sample = doc! {
        "name" : "Hossein",
        "email" : "spark_orm_test"
    };
    let re = user.find_one(Prototype::Doc(sample)).await;

    let founded = re.unwrap();

    if founded.is_some() {
        println!("The result {:?} ", founded.unwrap());
    } else {
        println!("The result is empty");
    }
}


async fn get_db() -> Arc<Database> {
    Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await
}


#[tokio::test]
async fn __find_one() {
    let db = get_db().await;
    let col = &db.collection::<Document>("users");
    let ds = doc! {
        "name" : "Hossein"
    };
    let re = col.find_one(ds, None).await;
    let res = re.unwrap();
    if res.is_some() {
        println!("the ")
    } else {
        println!("Nothing");
    }
}

#[tokio::test]
async fn find() {
    let db = get_db().await;
    let we = doc! {
        "name" : "Hossein"
    };  
    let mut raw_re = User::find(we , &db , "users").await.unwrap();
    while raw_re.advance().await.unwrap() {
        println!("the doc is {:?} " , raw_re.deserialize_current().unwrap());
    }
}