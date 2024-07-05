use crate::model::{Model, MongodbResult};

/// This trait implement by default for Model
/// but if user wants to override and use it must tell to Model macro
/// ```rust
/// #[Model(coll_name = "users", observer)]
/// #[derive(Serialize, Deserialize, Debug, Default)]
/// struct User {
///    name: String,
/// }
/// ```


#[allow(async_fn_in_trait)]
#[allow(unused)]
pub trait Observer<M> {

    /// this call when document is created , in these observers can't call save again
    async fn created(model: &mut Model<'_, M>) -> MongodbResult<()> {
        Ok(())
    }

    /// this call when document is updated
    /// it just called when user uses save method not update method
    async fn updated(model: &mut Model<'_, M>) -> MongodbResult<()> {
        Ok(())
    }

    /// this call when document is delete
    async fn deleted(model: &mut Model<'_, M>) -> MongodbResult<()> {
        Ok(())
    }
}
