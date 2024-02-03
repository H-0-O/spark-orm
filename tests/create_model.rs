use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use rm_orm::{RmORM};
use rm_orm::model::utility::inner_utility::InnerUtility;

use rm_orm_derive::Model;

#[Model(coll_name = "TestModels")]
#[derive(Debug , Serialize , Deserialize , Default)]
struct TestModel
{
    name: String,
    // #[serde()]
    // #[model(unique)]
    m_name: No,

}
#[derive(Debug , Serialize , Deserialize , Default)]
struct No {
    kk: String
}

#[tokio::test]
async fn create_model_test() -> Result<() , String> {
    let db = RmORM::global_connect("fdf"  , "ff" , "df" , "df" , "sfsd").await;
    let mut test_m = TestModel::new_model(&db);
    //
    // test_m.m_name = "dfdf".into();
    // let re = test_m.save().await?;
    // println!("Hello {:?} " , test_m);
    Ok(())
}
