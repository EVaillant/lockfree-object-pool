use crate::async_variants::mutex_owned_reusable::MutexOwnedReusable;
use crate::async_variants::mutex_reusable::MutexReusable;
use futures::executor::block_on;
use std::mem::ManuallyDrop;
use std::sync::{Arc, Mutex};

/// ObjectPool use a [`std::sync::Mutex`] over vector to secure multithread access to the pool.
/// # Example
/// ```rust
///  use lockfree_object_pool::async_variants::MutexObjectPool;
///  use futures::executor::block_on;
///
///  let pool = MutexObjectPool::new(
///    async ||  0,
///    async |v| {
///      *v = 0;
///    }
///  );
///  let mut item: &mut i32 = &mut block_on(pool.pull());
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct MutexObjectPool<
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    objects: Mutex<Vec<T>>,
    reset: R,
    init: I,
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > MutexObjectPool<T, R, I>
{
    ///
    /// Create a new [`MutexObjectPool`]
    ///
    /// # Arguments
    /// * `init`  closure to create new item
    /// * `reset` closure to reset item before reusage
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::MutexObjectPool;
    ///
    ///  let pool = MutexObjectPool::new(
    ///    async || 0,
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  );
    /// ```
    #[inline]
    pub fn new(init: I, reset: R) -> Self
    {
        Self {
            objects: Mutex::new(Vec::new()),
            reset,
            init,
        }
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::MutexObjectPool;
    ///
    ///  let pool = MutexObjectPool::new(
    ///    async || 0,
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  );
    ///  let mut item = pool.pull();
    /// ```
    #[inline]
    pub async fn pull(&self) -> MutexReusable<'_, T, R, I> {
        let option_t = self.objects.lock().unwrap().pop();
        MutexReusable::new(
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
    ///  use lockfree_object_pool::async_variants::MutexObjectPool;
    ///  use std::sync::Arc;
    ///
    ///  let pool = Arc::new(MutexObjectPool::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  ));
    ///  let mut item = pool.pull_owned();
    /// ```
    #[inline]
    pub async fn pull_owned(self: &Arc<Self>) -> MutexOwnedReusable<T, R, I> {
        let option_t = self.objects.lock().unwrap().pop();
        MutexOwnedReusable::new(
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
        self.objects.lock().unwrap().push(data);
    }
}
