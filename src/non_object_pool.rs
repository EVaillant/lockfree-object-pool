use crate::none_reusable::NoneReusable;

pub struct NoneObjectPool<T> {
    init: Box<dyn Fn() -> T>,
}

impl<T> NoneObjectPool<T> {
    pub fn new<I>(init: I) -> Self
    where
        I: Fn() -> T + 'static,
    {
        Self {
            init: Box::new(init),
        }
    }

    pub fn pull(&self) -> NoneReusable<T> {
        NoneReusable::new((self.init)())
    }
}

unsafe impl<T> Send for NoneObjectPool<T> {}
unsafe impl<T> Sync for NoneObjectPool<T> {}
