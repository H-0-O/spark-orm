use crate::model::BaseModel;

pub trait InnerUtility<T> {
    fn fill(&mut self, inner: T);
}

impl<'a, T> InnerUtility<T> for BaseModel<'a, T> {
    fn fill(&mut self, inner: T) {
        *self.inner = Some(inner);
    }
}
