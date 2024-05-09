use crate::model::model::ProxyModel;
use crate::model::InnerState;

pub trait InnerUtility<T> {
    fn fill(&mut self, inner: T);

    fn init(&mut self)
    where
        T: Default;

    fn restore_to_default(&mut self)
    where
        T: Default;

    fn set_or_default(&mut self, inner: Option<T>)
    where
        T: Default;
}

impl<'a, T> InnerUtility<T> for ProxyModel<'a, T> {
    fn fill(&mut self, inner: T) {
        self.inner = inner;
    }
    fn init(&mut self)
    where
        T: Default,
    {
        self.fill(T::default());
    }

    fn restore_to_default(&mut self)
    where
        T: Default,
    {
        self.set_inner_state(InnerState::Default);
        self.fill(T::default());
    }
    fn set_or_default(&mut self, inner: Option<T>)
    where
        T: Default,
    {
        match inner {
            Some(data) => self.fill(data),
            None => self.restore_to_default(),
        };
    }
}
