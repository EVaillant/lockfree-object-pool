use crate::async_variants::mutex_object_pool::MutexObjectPool;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

/// Wrapper over T used by [`MutexObjectPool`].
///
/// Access is allowed with [`std::ops::Deref`] or [`std::ops::DerefMut`]
/// # Example
/// ```rust
///  use lockfree_object_pool::async_variants::MutexObjectPool;
///  use futures::executor::block_on;
///
///  let pool = MutexObjectPool::new(
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
pub struct MutexReusable<
    'a,
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    pool: &'a MutexObjectPool<T, R, I>,
    data: ManuallyDrop<T>,
}

impl<
        'a,
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > MutexReusable<'a, T, R, I>
{
    /// Create new element
    ///
    /// # Arguments
    /// * `pool` object pool owner
    /// * `data` element to wrap
    #[inline]
    pub fn new(pool: &'a MutexObjectPool<T, R, I>, data: ManuallyDrop<T>) -> Self {
        Self { pool, data }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > DerefMut for MutexReusable<'_, T, R, I>
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
    > Deref for MutexReusable<'_, T, R, I>
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
    > Drop for MutexReusable<'_, T, R, I>
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
