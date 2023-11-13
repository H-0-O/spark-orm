use std::sync::{Arc, Mutex};
use mongodb::{Client, Database};
use crate::connection::{create_client, create_client_options};
use crate::error::RSparkError;
use crate::r_static::R_SPARK_STATIC;

pub type Result<T> = std::result::Result<T, RSparkError>;
#[derive(Debug)]
pub struct RSpark {
    client: Client,
    db: Database,

}
impl RSpark {
    pub async fn new(
        user_name: &str,
        password: &str,
        host: &str,
        port: &str,
        db_name: &str,
    ) -> Result<RSpark> {
        let options = create_client_options(user_name, password, host, port).await?;
        let client = create_client(options)?;
        let db = client.database(db_name);
        Ok(RSpark { client, db })
    }

    pub async fn create_global_instance(
        user_name: &str,
        password: &str,
        host: &str,
        port: &str,
        db_name: &str,
    ) {
        let r_spark_instance = RSpark::new(user_name, password, host, port, db_name)
            .await
            .unwrap();
        R_SPARK_STATIC.set(Arc::new(Mutex::new(r_spark_instance))).unwrap();
    }
    
}

