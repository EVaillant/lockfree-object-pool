use crate::spin_lock::SpinLock;
use crate::spin_lock_reusable::SpinLockReusable;
use std::mem::ManuallyDrop;

pub struct SpinLockObjectPool<T> {
    objects: SpinLock<Vec<T>>,
    reset: Box<dyn Fn(&mut T)>,
    init: Box<dyn Fn() -> T>,
}

impl<T> SpinLockObjectPool<T> {
    pub fn new<R, I>(init: I, reset: R) -> Self
    where
        R: Fn(&mut T) + 'static,
        I: Fn() -> T + 'static,
    {
        Self {
            objects: SpinLock::new(Vec::new()),
            reset: Box::new(reset),
            init: Box::new(init),
        }
    }

    pub fn pull(&self) -> SpinLockReusable<T> {
        SpinLockReusable::new(
            self,
            ManuallyDrop::new(self.objects.lock().pop().unwrap_or_else(&self.init)),
        )
    }

    pub fn attach(&self, mut data: T) {
        (self.reset)(&mut data);
        self.objects.lock().push(data);
    }
}

unsafe impl<T> Send for SpinLockObjectPool<T> {}
unsafe impl<T> Sync for SpinLockObjectPool<T> {}
