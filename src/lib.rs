pub use rspark_derive::*;

mod connection;
mod error;
pub mod model;
mod r_spark;
pub mod utilities;

pub type RSpark = r_spark::RSpark;
pub mod futures {
   pub use futures::*;
}
