use std::str::FromStr;
use std::sync::Arc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use spark_orm::Spark;
use spark_orm_derive::Model;

#[Model(coll_name = "users")]
#[derive(Serialize , Deserialize , Default , Debug)]
struct User {
    name: String,
    age: u64
}


//TODO test From trait with struct that has generic
#[Model(coll_name = "users")]
#[derive(Serialize , Deserialize , Default , Debug)]
struct Product {
    name: String,
}

// impl From<Product> for Document{
//     fn from(value: Product) -> Self {
//         let result = mongodb::bson::to_document(&value);
//         match result {
//             Ok(doc) => doc,
//             Err(_) => doc! {}
//         }
//     }
// }


#[tokio::test]
async fn save() {
    let db = get_db().await;
    let mut user_model = User::new_model(Some(&db));
    user_model.name = "Hossein".to_string();
    user_model.save(None).await.unwrap();
}

#[tokio::test]
async fn find_one(){
    let db = get_db().await;
    let user_model = User::new_model(Some(&db));
    let mut sample = User::default();
    sample.name = "Hossein".to_string();
    let founded = user_model.find_one(
        sample,
        None
    ).await.unwrap();
    println!("The founded object {:?} " , founded);

}

#[tokio::test]
async fn update(){
    let db = get_db().await;
    let mut user_model = User::new_model(Some(&db));
    user_model._id = Some(ObjectId::from_str("663a7a27cc6093d989a1e279").unwrap());
    user_model.name = "Hossein 2".to_string();
    user_model.age = 58;
    user_model.save(None).await.unwrap();
}

async fn get_db() -> Arc<Database> {
    Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await
}
