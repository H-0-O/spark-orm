use std::sync::Arc;

use mongodb::{Client, Database};
use once_cell::sync::OnceCell;

use crate::connection::{create_client, create_client_options};
use crate::RSpark;

pub(crate) static R_SPARK_STATIC: OnceCell<RSparkStaticType> = OnceCell::new();

#[derive(Debug)]
pub(crate) struct RSparkStaticType {
    client: Client,
    db: Arc<Database>,
}

impl RSparkStaticType {
    pub async fn create_global_instance(
        user_name: &str,
        password: &str,
        host: &str,
        port: &str,
        db_name: &str,
    ) {
        let client_options = create_client_options(user_name, password, host, port)
            .await
            .unwrap();
        let client = create_client(client_options).unwrap();
        let db = client.database(db_name);
        let rs = RSparkStaticType {
            client,
            db: Arc::new(db),
        };
        R_SPARK_STATIC.set(rs).unwrap();
    }
}

impl RSpark {
    pub async fn create_global(
        user_name: &str,
        password: &str,
        host: &str,
        port: &str,
        db_name: &str,
    ) -> Arc<Database> {
        RSparkStaticType::create_global_instance(user_name, password, host, port, db_name).await;
        RSpark::get_global_db()
    }

    pub fn get_global_db() -> Arc<Database> {
        let r_spark_instance = R_SPARK_STATIC.get().unwrap();
        Arc::clone(&r_spark_instance.db)
    }
}
