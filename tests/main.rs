use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use rspark::{model::{BaseModel, crud::BaseModelCrud, inner_crud::InnerCRUD}, Model, RSpark};
use rspark::model::inner_utility::InnerUtility;
use rspark::model::Prototype;

#[derive(Model, Serialize, Debug, Deserialize)]
#[coll_name = "Books"]
pub struct Book {
    #[model(unique)]
    name: String,
    author: String,
    the_type: String,
}



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
