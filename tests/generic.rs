use std::fmt::Debug;
use std::sync::Arc;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use spark_orm::Spark;
use spark_orm_derive::Model;

#[Model(coll_name = "users")]
#[derive(Serialize, Deserialize, Default, Debug)]
struct User<W>
{
    #[index]
    name: String,
    age: u64,
    collect: W,
}

#[derive(Deserialize ,Serialize , Default ,Debug )]
struct Product{
    name: String
}


// impl<W> From<User<W>> for Document
// where
//     W: DeserializeOwned,
//     W: Serialize,
//     W: Debug,
//     W: Unpin,
//     W: Sync,
//     W: Send,
//     W: Default
// {
//     fn from(value: User<W>) -> Self {
//         doc! {}
//     }
// }

// impl From<User<Product>> for Document{
//     fn from(value: User<Product>) -> Self {
//         doc! {}
//     }
// }
#[tokio::test]
async fn main_generic(){
    let db = get_db().await;
    let user = User::<Product>::new_model(Some(&db));
    let ud = User::<Product>::default();
    user.find(&ud , None).await.unwrap();
}
async fn get_db() -> Arc<Database> {
    Spark::global_connect("root", "123", "localhost", "6789", "rm_orm_db").await
}
