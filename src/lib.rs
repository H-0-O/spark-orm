pub use rspark_derive::*;
pub use model::TModel;
mod connection;
mod error;
mod model;
mod r_spark;
pub mod utilities;

pub type RSpark = r_spark::RSpark;
