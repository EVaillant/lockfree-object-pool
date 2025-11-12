use crate::async_variants::linear_owned_reusable::LinearOwnedReusable;
use crate::async_variants::linear_page::LinearPage;
use crate::async_variants::linear_reusable::LinearReusable;
use std::sync::Arc;

/// ObjectPool use a lockfree vector to secure multithread access to the pool.
///
/// The lockfree vector is implemented as linked list.
///
/// # Example
/// ```rust
///  use lockfree_object_pool::async_variants::LinearObjectPool;
///
///  let pool = LinearObjectPool::new(
///    async ||  Default::default(),
///    async |v| {
///      *v = 0;
///    }
///  );
///  let mut item = pool.pull();
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct LinearObjectPool<
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    reset: R,
    init: I,
    head: LinearPage<T, I>,
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > LinearObjectPool<T, R, I>
{
    ///
    /// Create a new [`LinearObjectPool`]
    ///
    /// # Arguments
    /// * `init`  closure to create new item
    /// * `reset` closure to reset item before reusage
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::LinearObjectPoolAsync;
    ///
    ///  let pool = LinearObjectPoolAsync::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  );
    /// ```
    #[inline]
    pub async fn new(init: I, reset: R) -> Self {
        Self {
            reset,
            init: init.clone(),
            head: LinearPage::new(init).await,
        }
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pool.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::LinearObjectPoolAsync;
    ///
    ///  let pool = LinearObjectPoolAsync::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  );
    ///  let mut item = pool.pull();
    /// ```
    #[inline]
    pub async fn pull(&self) -> LinearReusable<'_, T, R, I> {
        let (page, page_id) = self.head.alloc(self.init.clone()).await;
        unsafe { LinearReusable::new(self, page_id, page) }
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::LinearObjectPoolAsync;
    ///  use std::sync::Arc;
    ///
    ///  let pool = Arc::new(LinearObjectPoolAsync::new(
    ///    async ||  Default::default(),
    ///    async |v| {
    ///      *v = 0;
    ///    }
    ///  ));
    ///  let mut item = pool.pull_owned();
    /// ```
    #[inline]
    pub async fn pull_owned(self: &Arc<Self>) -> LinearOwnedReusable<T, R, I> {
        let (page, page_id) = self.head.alloc(self.init.clone()).await;
        unsafe { LinearOwnedReusable::new(self.clone(), page_id, page) }
    }

    #[inline]
    pub(crate) fn get_reset_callback(&self) -> R {
        self.reset.clone()
    }
}
