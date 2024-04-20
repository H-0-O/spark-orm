use std::sync::Arc;

use mongodb::{Client, Database, IndexModel};
use mongodb::bson::doc;
use once_cell::sync::OnceCell;
use serde::{Serialize};
use serde::de::DeserializeOwned;

use crate::connection::{create_client, create_client_options};
use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) static R_M_ORM_STATIC: OnceCell<Spark> = OnceCell::new();

#[derive(Debug)]
pub struct Spark {
    #[allow(dead_code)]
    client: Client,
    db: Arc<Database>,
}

impl Spark {
    pub async fn global_connect(
        user_name: &str,
        password: &str,
        host: &str,
        port: &str,
        db_name: &str,
    ) -> Arc<Database> {
        let client_options = create_client_options(user_name, password, host, port)
            .await
            .unwrap();
        let client = create_client(client_options).unwrap();
        // client
        //     .database("user")
        //     .run_command(doc! {"ping": 1}, None)
        //     .await?;
        let db = client.database(db_name);
        let rs = Spark {
            client,
            db: Arc::new(db),
        };
        R_M_ORM_STATIC.set(rs).unwrap();
        Self::get_db()
    }

    pub async fn connect(
        user_name: &str,
        password: &str,
        host: &str,
        port: &str,
        db_name: &str,
    ) -> Database {
        let client_options = create_client_options(user_name, password, host, port)
            .await
            .unwrap();
        let client = create_client(client_options).unwrap();
        client.database(db_name)
    }
    pub fn get_db() -> Arc<Database> {
        let spark = R_M_ORM_STATIC.get();
        match spark {
            Some(rs) => Arc::clone(&rs.db),
            None => panic!("The Data base not set !!!"),
        }
    }

    pub fn from_mongo_result<T>(re: mongodb::error::Result<T>) -> Result<T> {
        match re {
            Ok(inner_re) => Ok(inner_re),
            Err(error) => Err(Error::new(&error.to_string())),
        }
    }

    pub async fn register_attributes<Model>(db: &Database, attributes: Vec<&str>, coll_name: &str)
        where
            Model: Serialize,
            Model: DeserializeOwned,
            Model: Send,
            Model: Sync,
    {
        // let index_model = IndexModel::builder().keys(
        //     doc! {
        //         "user_name": 1
        //     }
        // ).build();
        // let coll = &db.collection::<Document>("users");
        // let result = coll.create_index(
        //     index_model,
        //     None,
        // ).await;
        println!("the attrs {attributes:?}");
        for attr in attributes {
            let att = attr.to_string();
            let index_model = IndexModel::builder().keys(
                doc! {
                 att: 1
             }
            ).build();
            let coll = db.collection::<Model>(coll_name);
            let re = coll.create_index(
                index_model,
                None,
            ).await.expect(
                &format!("Can't create attribute on {attr}")
            );
            println!("the result is {re:?}");
        }
       
    }
}
