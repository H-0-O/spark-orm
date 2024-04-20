use mongodb::bson::DateTime as MDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DateTime(MDateTime);

impl Default for DateTime {
    fn default() -> Self {
        DateTime(MDateTime::now())
    }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
