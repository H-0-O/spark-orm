use mongodb::Database;

use rm_orm::preload::*;
use rm_orm_derive::create_model;

#[create_model]
struct TestModel {
    name: String,
    l_name: String,
    m_name: String
}
#[tokio::test]
async fn create_model_test(){
    // let t = TestModel{
    //     _id: String::from("Hossein")
    // };
    let db = RmORM::global_connect("fdf"  , "ff" , "df" , "df" , "sfsd").await;
    let mut test_m = TestModel::new(&db).await;
    test_m.m_name = "dfdf".into();
    test_m.save().await.unwrap();
    // println!("Hello {:?} " , test_m);

}