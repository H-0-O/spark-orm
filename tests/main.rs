use mongodb::Database;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use rspark::{
    Model,
    model::{
        BaseModel,
        inner_crud::InnerCRUD,
        crud::BaseModelCrud
    },
    RSpark,
};

#[derive(Model, Serialize, Debug, Deserialize)]
#[coll_name = "Books"]
pub struct Book {
    #[model(unique)]
    name: String,
    author: String,
    the_type: String,
}
#[tokio::test]
async fn main_test() {
    let db = RSpark::connect("admin", "123", "localhost", "27019", "main_db").await;

}
// TODO create a thread test for testing global db in thread

#[tokio::test]
async fn test_all() {
    RSpark::global_connect("admin", "123", "localhost", "27019", "main_db").await;
    // Book::all_with_callback(|book|{
    //     println!("the Book name is {:?} " , book.name);
    // }).await;
}

#[tokio::test]
async fn _save(){
    let db = RSpark::connect("admin", "123", "localhost", "27019", "main_db").await;
    let the_book = Book::new(&db).await;
    let fs  = the_book.find_one_with_doc(doc! {}).await.unwrap();
    match fs {
        Some(da) => {
        },
        None => {}
    }
}