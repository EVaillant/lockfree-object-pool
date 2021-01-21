use crate::mutex_reusable::MutexReusable;
use std::mem::ManuallyDrop;
use std::sync::Mutex;

/// ObjectPool use a [`std::sync::Mutex`] over vector to secure multithread access to pull.
/// # Example
/// ```rust
///  use lockfree_object_pool::MutexObjectPool;
///
///  let pool = MutexObjectPool::<u32>::new(
///    ||  Default::default(),
///    |v| {
///      *v = 0;
///    }
///  );
///  let mut item = pool.pull();
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct MutexObjectPool<T> {
    objects: Mutex<Vec<T>>,
    reset: Box<dyn Fn(&mut T)>,
    init: Box<dyn Fn() -> T>,
}

impl<T> MutexObjectPool<T> {
    ///
    /// Create an new [`MutexObjectPool`]
    ///
    /// # Arguments
    /// * `init`  closure to create new item
    /// * `reset` closure to reset item before reusage
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::MutexObjectPool;
    ///
    ///  let pool = MutexObjectPool::<u32>::new(
    ///    ||  Default::default(),
    ///    |v| {
    ///      *v = 0;
    ///    }
    ///  );
    /// ```
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

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::MutexObjectPool;
    ///
    ///  let pool = MutexObjectPool::<u32>::new(
    ///    ||  Default::default(),
    ///    |v| {
    ///      *v = 0;
    ///    }
    ///  );
    ///  let mut item = pool.pull();
    /// ```
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

    #[doc(hidden)]
    pub fn attach(&self, mut data: T) {
        (self.reset)(&mut data);
        self.objects.lock().unwrap().push(data);
    }
}

unsafe impl<T> Send for MutexObjectPool<T> {}
unsafe impl<T> Sync for MutexObjectPool<T> {}
