use std::borrow::Borrow;

use mongodb::Database;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::DeserializeOwned;

use rm_orm::preload::*;
use rm_orm_derive::Model;

#[Model(coll_name = "TestModels")]
struct TestModel<F , T>
    where
    F: Default,
    F: Serialize,
    F : DeserializeOwned,
    T: Default,
    T: Serialize,
    T: DeserializeOwned
{
    name: String,
    #[serde()]
    #[model(unique)]
    l_name: F,
    m_name: String,
    t_w_w: T
}
#[derive(TModel , Default , Serialize , Deserialize)]
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
    f_name: T
}

#[tokio::test]
async fn create_model_test() -> Result<() , String> {
    let db = RmORM::global_connect("fdf"  , "ff" , "df" , "df" , "sfsd").await;
    let mut test_m = TestModel::new(&db).await;

    test_m.m_name = "dfdf".into();
    let re = test_m.save().await?;
    println!("Hello {:?} " , test_m);
    Ok(())
}
