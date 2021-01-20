use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct SpinLock<T> {
    data: T,
    lock: AtomicBool,
}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            lock: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        self.acquire();
        SpinLockGuard::new(self, &self.data)
    }

    fn acquire(&self) {
        self.exchange(false, true);
    }

    fn release(&self) {
        self.exchange(true, false);
    }

    fn exchange(&self, from: bool, to: bool) {
        loop {
            match self
                .lock
                .compare_exchange_weak(from, to, Ordering::SeqCst, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(_) => {
                    thread::yield_now();
                }
            }
        }
    }
}

unsafe impl<T> Send for SpinLock<T> {}
unsafe impl<T> Sync for SpinLock<T> {}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
    data: *const T,
}

impl<'a, T> SpinLockGuard<'a, T> {
    pub fn new(lock: &'a SpinLock<T>, data: *const T) -> Self {
        Self { lock, data }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (self.data as *mut T).as_mut().unwrap() }
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref().unwrap() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.release();
    }
}
