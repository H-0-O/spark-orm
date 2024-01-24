use std::borrow::Borrow;

use mongodb::Database;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::DeserializeOwned;

use rm_orm::preload::*;
use rm_orm_derive::create_model;

#[create_model]
struct TestModel {
    name: String,
    l_name: String,
    m_name: String,
}
#[derive(Model , Default , Serialize , Deserialize)]
#[coll_name = "df"]
struct NModel<T>
    where
        T: Default,
        T: Serialize,
        T : DeserializeOwned
{
    name: String,
    // #[serde(deserialize_with = "T::deserialize")]
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    fName: T
}

#[tokio::test]
async fn create_model_test() {
    // let t = TestModel{
    //     _id: String::from("Hossein")
    // };
    // let db = RmORM::global_connect("fdf"  , "ff" , "df" , "df" , "sfsd").await;
    // let mut test_m = TestModel::new(&db).await;
    // test_m.m_name = "dfdf".into();
    // test_m.save().await.unwrap();
    // println!("Hello {:?} " , test_m);
}
