#![allow(dead_code)]

mod connection;
mod error;
mod model;
mod r_spark;
pub(crate) mod r_static;
pub mod utilities;

pub type RSpark = r_spark::RSpark;

pub use crate::model::*;
pub use rspark_derive::*;
