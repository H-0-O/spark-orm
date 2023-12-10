use futures::future::err;
use mongodb::bson::doc;
use mongodb::Database;
use mongodb::results::InsertOneResult;
use serde::{Deserialize, Serialize};

use rspark::{model::{BaseModel, crud::BaseModelCrud, inner_crud::InnerCRUD}, Model, RSpark, RSparkResult};
use rspark::model::inner_utility::InnerUtility;
use rspark::model::Prototype;
use futures::{StreamExt, TryStreamExt};
#[derive(Model, Serialize, Debug, Deserialize)]
#[coll_name = "Books"]
pub struct Book {
    #[model(unique)]
    name: String,
    author: String,
    the_type: String,
}
// TODO move the tests into a tests/crud.rs file 
// TODO 
#[tokio::test]
async fn _save() {
    let db = get_test_db().await;
    let mut the_book = Book::new(&db).await;
    the_book.fill(get_test_book());
    the_book.save().await.unwrap();
}

#[tokio::test]
async fn __find(){
    let db = get_test_db().await;
    let the_book = Book::new(&db).await;
    let result =the_book.find(Prototype::Model(get_test_book())).await;
    match result { 
        Ok(mut stream) => {
            while let Some(res_doc) = stream.next().await {
                if let Ok(book) = res_doc {
                    println!(" The Book is {:?} " , book );
                }
            }
        },
        Err(error) => {
            panic!(error.to_string())
        }
    }
    
    println!("__find is passed ");
}

#[tokio::test]
async fn __find_one_with_doc(){
    let db = get_test_db().await;
    let the_book = Book::new(&db).await;
    let prototype = Prototype::Doc(
        doc!{
                "name": "SomeThing"
        }
    );
    let re = the_book.find_one(prototype).await;
}

async fn __find_one_with_model(){
    let db = get_test_db().await;
    let the_book = Book::new(&db).await;
    let prototype = Prototype::Model(get_test_book());
    let re = the_book.find_one(prototype).await;
}

fn get_test_book() -> Book{
    Book {
        name: "Hassan".to_string(),
        author: "Ali".to_string(),
        the_type: "__save".to_string(),
    }
}

async  fn get_test_db() -> Database{
    RSpark::connect("admin", "123", "localhost", "27019", "main_db").await
}
