use std::sync::Arc;

use mongodb::{Client, Database};
use once_cell::sync::OnceCell;

use crate::connection::{create_client, create_client_options};
use crate::error::RmORMError;

pub type RmORMResult<T> = std::result::Result<T, RmORMError>;
pub(crate) static R_M_ORM_STATIC: OnceCell<RmORM> = OnceCell::new();

#[derive(Debug)]
pub struct RmORM {
    #[allow(dead_code)]
    client: Client,
    db: Arc<Database>,
}

impl RmORM {
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
        let rs = RmORM {
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
        let r_spark = R_M_ORM_STATIC.get();
        match r_spark {
            Some(rs) => Arc::clone(&rs.db),
            None => panic!("The Data base not set !!!"),
        }
    }

    pub fn from_mongo_result<T>(re: mongodb::error::Result<T>) -> RmORMResult<T> {
        match re {
            Ok(inner_re) => Ok(inner_re),
            Err(error) => Err(RmORMError::new(&error.to_string())),
        }
    }
}
