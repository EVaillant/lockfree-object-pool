use crate::{
    async_variants::spin_lock_owned_reusable::SpinLockOwnedReusable,
    async_variants::spin_lock_reusable::SpinLockReusable, spin_lock::SpinLock,
};
use futures::executor::block_on;
use std::mem::ManuallyDrop;
use std::sync::Arc;

/// ObjectPool use a spin lock over vector to secure multithread access to the pool.
///
/// The spin lock works like [`std::sync::Mutex`] but
/// * use [`std::sync::atomic::AtomicBool`] for synchro
/// * active waiting
///
/// cf [wikipedia](https://en.wikipedia.org/wiki/Spinlock) for more information.
///
/// # Example
/// ```rust
///  use lockfree_object_pool::async_variants::SpinLockObjectPool;
///  use futures::executor::block_on;
///
///  let pool = SpinLockObjectPool::new(
///    async ||  Default::default(),
///    async |v| {
///      *v = 0;
///    }
///  );
///  let mut item: &mut i32 = &mut block_on(pool.pull());
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct SpinLockObjectPool<
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    objects: SpinLock<Vec<T>>,
    reset: R,
    init: I,
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > SpinLockObjectPool<T, R, I>
{
    ///
    /// Create a new [`SpinLockObjectPool`]
    ///
    /// # Arguments
    /// * `init`  closure to create new item
    /// * `reset` closure to reset item before reusage
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::SpinLockObjectPool;
    ///
    ///  let pool = SpinLockObjectPool::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  );
    /// ```
    #[inline]
    pub fn new(init: I, reset: R) -> Self
    {
        Self {
            objects: SpinLock::new(Vec::new()),
            reset,
            init,
        }
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::SpinLockObjectPool;
    ///
    ///  let pool = SpinLockObjectPool::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  );
    ///  let mut item = pool.pull();
    /// ```
    #[inline]
    pub async fn pull(&self) -> SpinLockReusable<'_, T, R, I> {
        let option_t = self.objects.lock().pop();
        SpinLockReusable::new(
            self,
            ManuallyDrop::new(match option_t {
                Some(object) => object,
                None => (self.init)().await,
            }),
        )
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::SpinLockObjectPool;
    ///  use std::sync::Arc;
    ///
    ///  let pool = Arc::new(SpinLockObjectPool::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  ));
    ///  let mut item = pool.pull_owned();
    /// ```
    #[inline]
    pub async fn pull_owned(self: &Arc<Self>) -> SpinLockOwnedReusable<T, R, I> {
        let option_t = self.objects.lock().pop();
        SpinLockOwnedReusable::new(
            self.clone(),
            ManuallyDrop::new(match option_t {
                Some(object) => object,
                None => (self.init)().await,
            }),
        )
    }

    #[inline]
    pub(crate) fn attach(&self, mut data: T) {
        block_on((self.reset)(&mut data));
        self.objects.lock().push(data);
    }
}
