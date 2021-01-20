use crate::spin_lock_object_pool::SpinLockObjectPool;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct SpinLockReusable<'a, T> {
    pool: &'a SpinLockObjectPool<T>,
    data: ManuallyDrop<T>,
}

impl<'a, T> SpinLockReusable<'a, T> {
    pub fn new(pool: &'a SpinLockObjectPool<T>, data: ManuallyDrop<T>) -> Self {
        Self { pool, data }
    }
}

impl<'a, T> DerefMut for SpinLockReusable<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> Deref for SpinLockReusable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> Drop for SpinLockReusable<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.pool
                .attach(ManuallyDrop::into_inner(ptr::read(&self.data)));
        }
    }
}
