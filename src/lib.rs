#![allow(dead_code)]
pub mod model;
mod r_spark;
pub(crate) mod r_static;
mod connection;
mod error;
pub type RSpark = r_spark::RSpark;

