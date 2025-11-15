use crate::async_variants::mutex_object_pool::MutexObjectPool;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Wrapper over T used by [`MutexObjectPool`].
///
/// Access is allowed with [`std::ops::Deref`] or [`std::ops::DerefMut`]
/// # Example
/// ```rust
///  use lockfree_object_pool::async_variants::MutexObjectPool;
///  use std::sync::Arc;
///  use futures::executor::block_on;
///
///  let pool = Arc::new(MutexObjectPool::new(
///    async ||  Default::default(),
///    async |v| {
///      *v = 0;
///    }
///  ));
///  let mut item: &mut i32 = &mut block_on(pool.pull_owned());
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct MutexOwnedReusable<
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    pool: Arc<MutexObjectPool<T, R, I>>,
    data: ManuallyDrop<T>,
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > MutexOwnedReusable<T, R, I>
{
    /// Create new element
    ///
    /// # Arguments
    /// * `pool` object pool owner
    /// * `data` element to wrap
    #[inline]
    pub fn new(pool: Arc<MutexObjectPool<T, R, I>>, data: ManuallyDrop<T>) -> Self {
        Self { pool, data }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > DerefMut for MutexOwnedReusable<T, R, I>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Deref for MutexOwnedReusable<T, R, I>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Drop for MutexOwnedReusable<T, R, I>
{
    #[inline]
    fn drop(&mut self) {
        let data = unsafe {
            // SAFETY: self.data is never referenced again and it isn't dropped
            ManuallyDrop::take(&mut self.data)
        };
        self.pool.attach(data);
    }
}
