use crate::mutex_reusable::MutexReusable;
use std::mem::ManuallyDrop;
use std::sync::Mutex;

pub struct MutexObjectPool<T> {
    objects: Mutex<Vec<T>>,
    reset: Box<dyn Fn(&mut T)>,
    init: Box<dyn Fn() -> T>,
}

impl<T> MutexObjectPool<T> {
    pub fn new<R, I>(init: I, reset: R) -> Self
    where
        R: Fn(&mut T) + 'static,
        I: Fn() -> T + 'static,
    {
        Self {
            objects: Mutex::new(Vec::new()),
            reset: Box::new(reset),
            init: Box::new(init),
        }
    }

    pub fn pull(&self) -> MutexReusable<T> {
        MutexReusable::new(
            self,
            ManuallyDrop::new(
                self.objects
                    .lock()
                    .unwrap()
                    .pop()
                    .unwrap_or_else(&self.init),
            ),
        )
    }

    pub fn attach(&self, mut data: T) {
        (self.reset)(&mut data);
        self.objects.lock().unwrap().push(data);
    }
}

unsafe impl<T> Send for MutexObjectPool<T> {}
unsafe impl<T> Sync for MutexObjectPool<T> {}
