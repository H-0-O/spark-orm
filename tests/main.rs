use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use mongodb::bson::{doc};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use spark_orm::Spark;
use spark_orm_derive::Model;

#[Model(coll_name = "users")]
#[derive(Serialize, Deserialize, Default, Debug)]
struct User
{
    #[index]
    name: String,
    age: u64,
    collect: Product
}


//TODO test From trait with struct that has generic
#[derive(Serialize , Deserialize , Default , Debug)]
struct Product {
    name: String,
}




#[tokio::test]
async fn save() {
    let db = get_db().await;
    let mut user_model = User::new_model(Some(&db));
    user_model.name = "Hossein".to_string();
    user_model.save(None).await.unwrap();
}

#[tokio::test]
async fn find_one() {
    let db = get_db().await;
    let user_model = User::new_model(Some(&db));
    let mut sample = User::default();
    sample.name = "Hossein".to_string();
    let founded = user_model.find_one(
        sample,
        None,
    ).await.unwrap();
    println!("The founded object {:?} ", founded);
}

#[tokio::test]
async fn update() {
    let db = get_db().await;
    let mut user_model = User::new_model(Some(&db));
    user_model._id = Some(ObjectId::from_str("663a7a27cc6093d989a1e279").unwrap());
    user_model.name = "Hossein 2".to_string();
    user_model.age = 58;
    user_model.save(None).await.unwrap();
}

#[tokio::test]
async fn update_with_doc() {
    let db = get_db().await;
    let user_model = User::new_model(Some(&db));
    let updated = user_model.update(
        doc! {
            "name": "Hossein",
        },
        doc! {
            "$set": {
                "name": "Hossein 33"
            }
        },
        None,
    ).await.unwrap();
    println!("The Updated info {:?}", updated);
}

#[tokio::test]
async fn update_with_model() {
    let db = get_db().await;
    let user_model = User::new_model(Some(&db));
    let mut sample_user = User::default();
    sample_user.name = "Hossein 33".to_string();
    let updated = user_model.update(
        &sample_user,
        doc! {
            "$set": {
                "name": "Hossein 3355"
            }
        },
        None,
    ).await.unwrap();

    println!("The Updated info {:?}", updated);
}

#[tokio::test]
async fn update_with_model_instance() {
    let db = get_db().await;
    let mut user_model = User::new_model(Some(&db));
    user_model.name = "Hossein 3355".to_string();
    user_model.age = 58;
    let updated = user_model.update(
        &user_model,
        doc! {
            "$set": {
                "name": "Hossein 325"
            }
        },
        None,
    ).await.unwrap();
    println!("The Updated info {:?}", updated);
}

#[tokio::test]
async fn find_and_collect() {
    let db = get_db().await;
    let user_model = User::new_model(Some(&db));

    let users = user_model.find_and_collect(
        doc! {"name": "Hossein 2"},
        None,
    ).await.unwrap();
    
    println!("The users {users:?} ")
}


async fn get_db() -> Arc<Database> {
    Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await
}
