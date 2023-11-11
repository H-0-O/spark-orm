use std::sync::{Arc, Mutex};

use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use once_cell::sync::OnceCell;

pub type Result<T> = std::result::Result<T, RoMMError>;
static RO_MM_GLOBAL: OnceCell<Arc<Mutex<RoMM>>> = OnceCell::new();

#[derive(Debug)]
pub struct RoMM {
    client: Client,
    db: Database,
}

#[derive(Debug)]
pub struct RoMMError {
    message: String,
}


impl RoMM {
    pub async fn new(user_name: &str, password: &str, host: &str, port: &str , db_name: &str) -> Result<RoMM> {
        let options = RoMM::create_client_options(user_name, password, host, port).await?;
        let client = RoMM::create_client(options)?;
        let db = client.database(db_name);
        Ok(
            RoMM{
                client,
                db
            }
        )

    }

    pub async fn create_global_instance(user_name: &str, password: &str, host: &str, port: &str , db_name: &str){
        let ro_mm = RoMM::new(user_name, password, host, port , db_name ).await.unwrap();
        RO_MM_GLOBAL.set(
            Arc::new(
                Mutex::new(ro_mm)
            )
        ).unwrap();
    }

    async fn create_client_options(user_name: &str, password: &str, host: &str, port: &str) -> Result<ClientOptions> {
        let connection_string = format!(
            "mongodb://{}:{}@{}:{}",
            user_name, password, host, port
        );
        let client_options = ClientOptions::parse(connection_string).await;
        return match client_options {
            Ok(otp) => Ok(otp),
            Err(err) => panic!("can not create an option")
        };
    }

    fn create_client(options: ClientOptions) -> Result<Client> {
        Ok(Client::with_options(options).unwrap())
    }
}


impl RoMMError {
    pub fn new(message: &str) -> RoMMError {
        RoMMError {
            message: message.to_string()
        }
    }
}

