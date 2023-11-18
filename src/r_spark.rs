use mongodb::bson::doc;
use mongodb::options::IndexOptions;
use mongodb::{Client, Database, IndexModel};

use crate::connection::{create_client, create_client_options};
use crate::error::RSparkError;

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

    pub fn get_db(&self) -> &Database {
        &self.db
    }
}
