use std::sync::Arc;
use mongodb::bson::{doc, Document};
use mongodb::{Database, IndexModel};
use serde::{Deserialize, Serialize};
use spark_orm::{Model, ProxyModelCrud, Spark};
use spark_orm::model::crud::inner_crud::InnerCRUD;
use spark_orm::model::Prototype;

#[Model(coll_name = "users")]
#[derive(Serialize, Deserialize, Default, Debug)]
struct User {
    #[index]
    age: u32,
    #[index]
    name: String,
    email: String,
}

#[Model(coll_name = "products")]
#[derive(Serialize, Deserialize, Default, Debug)]
struct Product {
    #[index]
    age: u32,
    #[index]
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
    let mut raw_re = User::find(we, &db, "users").await.unwrap();
    while raw_re.advance().await.unwrap() {
        println!("the doc is {:?} ", raw_re.deserialize_current().unwrap());
    }
}

#[tokio::test]
async fn find_with_callback() {
    let db = get_db().await;

    let we = doc! {
        "name" : "Hossein"
    };
    User::find_with_callback(
        we,
        |f| {
            println!("the User is {:?}", f.name);
        },
        &db,
        "users",
    ).await;
}

#[tokio::test]
async fn soft_delete(){
    todo!()
}
#[tokio::test]
async fn force_delete(){
    todo!()
}

async fn on_create(){
    todo!()
}

async fn on_created(){
    todo!()
}

#[tokio::test]
async fn index_attribute(){
    let db = get_db().await;
    User::new_model(&db);
    User::new_model(&db);
    Product::new_model(&db);
    Product::new_model(&db);
    // let index_model = IndexModel::builder().keys(
    //     doc! {
    //         "user_name": 1
    //     }
    // ).build();
    // let coll = &db.collection::<Document>("users");
    // let result = coll.create_index(
    //     index_model,
    //     None,
    // ).await;

    // println!(
    //     "the result {result:?}"
    // );

    // struct User{
    //     #[index]
    //     user_name: String
    // }
}

async fn unique_attribute(){
    // struct User{
    //     // TODO in parenthesis must write error message
    //     #[unique (" the user name :user_name is exists ") ]
    //     user_name: String
    // }
}


