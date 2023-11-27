use std::sync::Arc;

use mongodb::{Client, Database};
use once_cell::sync::OnceCell;

use crate::connection::{create_client, create_client_options};
use crate::error::RSparkError;

pub type Result<T> = std::result::Result<T, RSparkError>;
pub(crate) static R_SPARK_STATIC: OnceCell<RSpark> = OnceCell::new();

#[derive(Debug)]
pub struct RSpark {
    client: Client,
    db: Arc<Database>,
}

impl RSpark {
    pub async fn connect(user_name: &str, password: &str, host: &str, port: &str, db_name: &str)  -> Arc<Database>{
        let client_options = create_client_options(user_name, password, host, port)
            .await
            .unwrap();
        let client = create_client(client_options).unwrap();
        let db = client.database(db_name);
        let rs = RSpark {
            client,
            db: Arc::new(db),
        };
        R_SPARK_STATIC.set(rs).unwrap();
        Self::get_db()
    }

    pub fn get_db() -> Arc<Database> {
        let r_spark = R_SPARK_STATIC.get();
        match r_spark {
            Some(rs) => Arc::clone(&rs.db),
            None => panic!("The Data base not set !!!"),
        }
    }
}
