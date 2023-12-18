pub use rm_orm_derive::*;

mod connection;
mod error;
pub mod model;
mod rm_orm;
pub mod utilities;

pub type RSpark = rm_orm::RmORM;

// NoneFn<T>::None it's instead of None::<fn(_)>
pub type NoneFn<T> = Option<fn(T)>;
pub mod futures {
   pub use futures::*;
}

pub use rm_orm::RSparkResult;
