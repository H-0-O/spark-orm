pub use spark_orm_derive::*;

extern crate self as spark_orm;

mod connection;
pub mod error;
pub mod model;
pub mod types;
pub mod client;
mod macros;

pub mod futures {
    pub use futures::*;
}
pub use preload::*;
pub mod preload {
    pub use spark_orm::error::Error;
    pub use spark_orm::{Model};
    pub use spark_orm::client::{Result, Spark};
}
