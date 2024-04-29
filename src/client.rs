use std::sync::Arc;
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
        let attrs = attributes.iter().map(|attr| attr.to_string()).collect::<Vec<String>>();

        trace!("Spawn task to register indexes");

        let handlers = tokio::spawn(async move {
            let coll = db.collection::<Model>(&coll_name);
            let mut previous_indexes = coll.list_indexes(None).await.unwrap();
            // println!("Hello {:?} ", previous_indexes);

            // Iterate over existing indexes
            while let Some(Ok(prev_index)) = previous_indexes.next().await {
                println!("Index keys: {}", prev_index.keys);
                let key = prev_index.keys.to_string();

                // Check if the index key exists in the list of attributes
                if !attrs.contains(&key) {
                    // If not, drop the index
                    match coll.drop_index(&key, None).await {
                        Ok(_) => println!("Index {} dropped successfully", key),
                        Err(e) => {
                            eprintln!("Failed to drop index {}: {:?}", key, e);
                            // Log the error
                            error!("Failed to drop index {}: {:?}", key, e);
                        }
                    }
                }
            }
            // for prev_index in previous_indexes.next() {
            //     if !attrs.contains(&prev_index) {
            //         trace!("Remove the {prev_index} index from collection {coll_name} ");
            //
            //         if let Err(error) = coll.drop_index(prev_index, None).await {
            //             println!("the error {} "  ,  error);
            //             error!("Can't remove previous index : {} " , error);
            //         }
            //     }
            // }
            let attrs = attrs.iter()
                .map(|attr| {
                    let key = attr.to_string();
                    IndexModel::builder().keys(
                        doc! {
                        key : 1
                }
                    ).build()
                }).collect::<Vec<IndexModel>>();

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
        });
        
        //     let s_attributes = Arc::new(attributes.iter().map(|&sd| sd.to_string()).collect::<Vec<_>>());
        //
        //
        //     println!("Indexes {:?} " , s_attributes );
        //
        //     trace!("Spawn thread to create index");
        //
        //     tokio::spawn(async move {
        //         let coll = db.collection::<Model>(&coll_name);
        //
        //         trace!("Drop the previous indexes");
        //         let mut previous_indexes = coll.list_indexes(None).await.unwrap();
        //
        //         while let Some(doc_result) = previous_indexes.next().await {
        //             let doc = doc_result.unwrap(); // Unwrap the Result to get the document
        //             let missing_attributes: Vec<_> = doc
        //                 .keys
        //                 .iter()
        //                 .filter(|pre_index| {
        //                     // Exclude "_id" from the attributes
        //                     *pre_index.0 != "_id" && !s_attributes.contains(&pre_index.0)
        //                 })
        //                 .map(|pre_index| pre_index.0)
        //                 .collect();
        //
        //             println!("Attributes not in s_attributes: {:?}", missing_attributes);
        //         }
        //
        //         //TODO just remove those indexes that not exists now
        //         // if let Err(error) = coll.drop_indexes(None).await {
        //         //     error!(
        //         //         "Can't remove the previous index : {}",
        //         //         error
        //         //     );
        //         // }
        //         s_attributes.iter().for_each(|attr|{
        //
        //             let index_model = IndexModel::builder().keys(
        //                 doc! {
        //                    attr: 1
        //                }
        //             ).build();
        //
        //             trace!("Create the index : {} ", attr);
        //             let re = async {
        //                 let create_index_result = coll.create_index(
        //                     index_model,
        //                     None,
        //                 ).await;
        //                 println!("THE ATTR {} ", attr);
        //                 if let Err(error) = create_index_result {
        //                     error!(
        //                     "Can't create index : {}",
        //                     error
        //                 );
        //                 }
        //             };
        //             tokio::spawn(re);
        //         });
        //     });
        //
        //     // tokio::spawn(async |x| {
        //     //     let coll = db.collection::<Model>(coll_name);
        //     //     println!("enter in scoped thread");
        //     //
        //     //     println!("finish the scoped thread");
        //     // });
    }

    
    fn init_logger(){
        env_logger::init();    
    }

}
