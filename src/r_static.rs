use std::sync::{Arc, Mutex};

use once_cell::sync::OnceCell;
use crate::r_spark::RSpark;

pub static R_SPARK_STATIC: OnceCell<Arc<Mutex<RSpark>>> = OnceCell::new();
