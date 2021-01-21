use crate::spin_lock::SpinLock;
use crate::spin_lock_reusable::SpinLockReusable;
use std::mem::ManuallyDrop;

/// ObjectPool use a spin lock over vector to secure multithread access to pull.
///
/// The spin lock works like [`std::sync::Mutex`] but 
/// * use [`std::sync::atomic::AtomicBool`] for synchro
/// * active waiting
/// 
/// cf [wikipedia](https://en.wikipedia.org/wiki/Spinlock) for more information.
/// 
/// # Example
/// ```rust
///  use lockfree_object_pool::SpinLockObjectPool;
///
///  let pool = SpinLockObjectPool::<u32>::new(
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
pub struct SpinLockObjectPool<T> {
    objects: SpinLock<Vec<T>>,
    reset: Box<dyn Fn(&mut T)>,
    init: Box<dyn Fn() -> T>,
}

impl<T> SpinLockObjectPool<T> {
    ///
    /// Create an new [`SpinLockObjectPool`]
    ///
    /// # Arguments
    /// * `init`  closure to create new item
    /// * `reset` closure to reset item before reusage
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::SpinLockObjectPool;
    ///
    ///  let pool = SpinLockObjectPool::<u32>::new(
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
            objects: SpinLock::new(Vec::new()),
            reset: Box::new(reset),
            init: Box::new(init),
        }
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::SpinLockObjectPool;
    ///
    ///  let pool = SpinLockObjectPool::<u32>::new(
    ///    ||  Default::default(),
    ///    |v| {
    ///      *v = 0;
    ///    }
    ///  );
    ///  let mut item = pool.pull();
    /// ```
    pub fn pull(&self) -> SpinLockReusable<T> {
        SpinLockReusable::new(
            self,
            ManuallyDrop::new(self.objects.lock().pop().unwrap_or_else(&self.init)),
        )
    }

    #[doc(hidden)]
    pub fn attach(&self, mut data: T) {
        (self.reset)(&mut data);
        self.objects.lock().push(data);
    }
}

unsafe impl<T> Send for SpinLockObjectPool<T> {}
unsafe impl<T> Sync for SpinLockObjectPool<T> {}
