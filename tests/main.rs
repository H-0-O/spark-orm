use mongodb::bson::doc;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use rspark::model::inner_utility::InnerUtility;
use rspark::{model::{crud::BaseModelCrud, inner_crud::InnerCRUD, BaseModel}, Model, NoneFn, RSpark};

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
    the_book.fill(get_book_test());
    the_book.save().await.unwrap();
}

#[tokio::test]
async fn __find(){
    let db = get_test_db().await;
    let the_book = Book::new(&db).await;
    let my_fn = |book : Book |{
        println!("THe book name is {:?} " , book.name);
    };
    let fs = the_book.find(Some(get_book_test()) , Some(my_fn)  ).await;
}

fn get_book_test() -> Book{
    Book {
        name: "Hassan".to_string(),
        author: "Ali".to_string(),
        the_type: "__save".to_string(),
    }
}

async  fn get_test_db() -> Database{
    RSpark::connect("admin", "123", "localhost", "27019", "main_db").await
}
