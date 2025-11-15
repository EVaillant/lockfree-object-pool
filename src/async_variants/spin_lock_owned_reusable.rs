use crate::async_variants::spin_lock_object_pool::SpinLockObjectPool;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Wrapper over T used by [`SpinLockObjectPool`].
///
/// Access is allowed with [`std::ops::Deref`] or [`std::ops::DerefMut`]
/// # Example
/// ```rust
///  use lockfree_object_pool::SpinLockObjectPool;
///  use std::sync::Arc;
///
///  let pool = Arc::new(SpinLockObjectPool::<u32>::new(
///    ||  Default::default(),
///    |v| {
///      *v = 0;
///    }
///  ));
///  let mut item = pool.pull_owned();
///
///  *item = 5;
///  let work = *item * 5;
/// ```
pub struct SpinLockOwnedReusable<
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    pool: Arc<SpinLockObjectPool<T, R, I>>,
    data: ManuallyDrop<T>,
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > SpinLockOwnedReusable<T, R, I>
{
    /// Create new element
    ///
    /// # Arguments
    /// * `pool` object pool owner
    /// * `data` element to wrappe
    #[inline]
    pub fn new(pool: Arc<SpinLockObjectPool<T, R, I>>, data: ManuallyDrop<T>) -> Self {
        Self { pool, data }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > DerefMut for SpinLockOwnedReusable<T, R, I>
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
    > Deref for SpinLockOwnedReusable<T, R, I>
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
    > Drop for SpinLockOwnedReusable<T, R, I>
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
