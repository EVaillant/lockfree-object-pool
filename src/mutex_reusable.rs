use crate::mutex_object_pool::MutexObjectPool;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct MutexReusable<'a, T> {
    pool: &'a MutexObjectPool<T>,
    data: ManuallyDrop<T>,
}

impl<'a, T> MutexReusable<'a, T> {
    pub fn new(pool: &'a MutexObjectPool<T>, data: ManuallyDrop<T>) -> Self {
        Self { pool, data }
    }
}

impl<'a, T> DerefMut for MutexReusable<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> Deref for MutexReusable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> Drop for MutexReusable<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.pool
                .attach(ManuallyDrop::into_inner(ptr::read(&self.data)));
        }
    }
}
