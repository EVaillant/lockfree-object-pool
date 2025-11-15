use crate::async_variants::none_reusable::NoneReusable;
use std::sync::Arc;

/// Basic allocation without pull. Used to compare default rust allocation with different kind of object pool.
/// # Example
/// ```rust
///  use lockfree_object_pool::async_variants::NoneObjectPool;
///  use futures::executor::block_on;
///
///  let pool = NoneObjectPool::new(async || 0);
///  let mut item: &mut i32 = &mut block_on(pool.pull());
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct NoneObjectPool<T, I: AsyncFn() -> T + Send + Sync + Clone + 'static> {
    init: I,
}

impl<T, I: AsyncFn() -> T + Send + Sync + Clone + 'static> NoneObjectPool<T, I> {
    ///
    /// Create a new [`NoneObjectPool`]
    ///
    /// # Arguments
    /// * `init` closure to create new item
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::NoneObjectPool;
    ///
    ///  let pool = NoneObjectPool::new(async || 0);
    /// ```
    #[inline]
    pub fn new(init: I) -> Self
    {
        Self { init }
    }

    ///
    /// Create a new element. When the element is dropped, it doesn't return in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use lockfree_object_pool::async_variants::NoneObjectPool;
    ///  use futures::executor::block_on;
    ///
    ///  let pool = NoneObjectPool::new(async || 0);
    ///  let mut item: &mut i32 = &mut block_on(pool.pull());
    /// ```
    #[inline]
    pub async fn pull(&self) -> NoneReusable<T> {
        NoneReusable::new((self.init)().await)
    }

    ///
    /// Create a new element. When the element is dropped, it doesn't return in the pull.
    ///
    /// # Example
    /// ```rust
    ///  use std::sync::Arc;
    ///  use lockfree_object_pool::async_variants::NoneObjectPool;
    ///  use futures::executor::block_on;
    ///
    ///  let pool = Arc::new(NoneObjectPool::new(async || 0));
    ///  let mut item: &mut i32 = &mut block_on(pool.pull_owned());
    /// ```
    #[inline]
    pub async fn pull_owned(self: &Arc<Self>) -> NoneReusable<T> {
        NoneReusable::new((self.init)().await)
    }
}
