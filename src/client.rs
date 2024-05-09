use std::sync::Arc;
use std::time::Duration;
use spark_orm::macros::{error, trace};
use mongodb::{Client, Database, IndexModel};
use mongodb::bson::doc;
use once_cell::sync::OnceCell;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use futures::StreamExt;
use mongodb::options::{DropIndexOptions, ListIndexesOptions};

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
            Some(rs) => rs.db.clone(),
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
        let max_time_to_drop = Some(Duration::from_secs(5));
        let (tx, _) = tokio::sync::oneshot::channel();

        trace!("Spawn task to register indexes");
        let register_attrs = async move {
            let coll = db.collection::<Model>(&coll_name);
            let previous_indexes = coll.list_indexes(
                Some(
                    ListIndexesOptions::builder().max_time(
                        max_time_to_drop
                    ).build()
                )
            ).await;
            if let Err(error) = previous_indexes {
                error!(
                    "Can't unpack the previous_indexes {error}"
                );
                return;
            }
            let mut keys_to_remove = Vec::new();
            let foreach_future = previous_indexes.unwrap().for_each(|pr| {
                match pr {
                    Ok(index_model) => {
                        index_model.keys.iter().for_each(|key| {
                            if key.0 != "_id" {
                                if let Some(pos) = attrs.iter().position(|k| k == key.0) {
                                    // means attribute exists in struct and database and not need to create it
                                    attrs.remove(pos);
                                } else if let Some(rw) = &index_model.options {
                                    // means the attribute must remove because not exists in struct
                                    keys_to_remove.push(
                                        rw.name.clone()
                                    )
                                }
                            }
                        });
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
                let _ = coll.drop_index(key,
                                        Some(
                                            DropIndexOptions::builder().max_time(
                                                max_time_to_drop
                                            ).build()
                                        ),
                ).await;
            }
            if !attrs.is_empty() {
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
            }
        };

        let task = tokio::spawn(register_attrs);

        let wait_for_complete = async move {
            let _ = task.await;
            let _ = tx.send(());
        };

        tokio::task::spawn(wait_for_complete);
    }

    pub fn use_db(){
        
    }
    fn init_logger() {
        env_logger::init();
    }
}
