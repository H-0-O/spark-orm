use rm_orm_derive::create_model;

#[create_model]
struct TestModel{
    id: String
}
#[test]
fn create_model_test(){
    // let t = TestModel{
    //     _id: String::from("Hossein")
    // };
    println!("Hello");
}