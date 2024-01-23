use serde::Deserializer;

use rm_orm::preload::*;
use rm_orm_derive::create_model;

#[create_model]
struct TestModel {
    name: String,
    l_name: String,
    m_name: String
}
#[test]
fn create_model_test(){
    // let t = TestModel{
    //     _id: String::from("Hossein")
    // };
    let test_m = TestModel::default();

    println!("Hello {:?} " , test_m);

}