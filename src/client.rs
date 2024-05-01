use std::future::Future;
use std::sync::Arc;
use futures::executor::block_on;
use spark_orm::macros::{error, trace};
use mongodb::{Client, Database, IndexModel};
use mongodb::bson::doc;
use once_cell::sync::OnceCell;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use futures::StreamExt;

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
        Self::init_logger();
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
        Self::init_logger();
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

    pub fn register_attributes<Model>(db: Arc<Database>, attributes: Vec<&str>, coll_name: String)
        where
            Model: Serialize,
            Model: DeserializeOwned,
            Model: Send,
            Model: Sync,
    {
        let mut attrs = attributes.iter().map(|attr| attr.to_string()).collect::<Vec<String>>();

        trace!("Spawn task to register indexes");
        let register_attrs = async move {
            let coll = db.collection::<Model>(&coll_name);
            let previous_indexes = coll.list_indexes(None).await.unwrap();
            let mut keys_to_remove = Vec::new();
            let foreach_future = previous_indexes.for_each(|pr| {
                match pr {
                    Ok(index_model) => {
                        for key in index_model.keys {
                            if key.0 == "_id" { continue; }
                            if let Some(pos) = attrs.iter().position(|k| k == &key.0) {
                                // means attribute exists in struct and database and not need to create it
                                attrs.remove(pos);
                            } else if let Some(rw) = &index_model.options {
                                // means the attribute must remove because not exists in struct
                                keys_to_remove.push(
                                    rw.name.clone()
                                )
                            }
                        }
                    }
                    Err(error) => {
                        error!(
                            "Can't unpack index model {error}"
                        );
                    }
                }
                futures::future::ready(())
            });
            foreach_future.await;

            let attrs = attrs.iter()
                .map(|attr| {
                    let key = attr.to_string();
                    IndexModel::builder().keys(
                        doc! {
                        key : 1
                }
                    ).build()
                }).collect::<Vec<IndexModel>>();

            for name in keys_to_remove {
                let key = name.as_ref().unwrap();
                coll.drop_index(key, None).await.unwrap();
            }
            let result = coll.create_indexes(
                attrs,
                None,
            ).await;

            if let Err(error) = result {
                error!(
                    "Can't create indexes : {:?}" ,
                    error
                );
            }
        };
        
        // let wq = tokio::task::spawn(register_attrs);
        
        // let bui = tokio::runtime::Builder::new_current_thread().thread_name("hello").build();

        // bui.as_ref().unwrap().block_on(register_attrs);

    }


    fn init_logger() {
        env_logger::init();
    }
}
