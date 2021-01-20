use std::ops::{Deref, DerefMut};

pub struct NoneReusable<T> {
    data: T,
}

impl<T> NoneReusable<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> DerefMut for NoneReusable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> Deref for NoneReusable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
