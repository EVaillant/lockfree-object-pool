use crate::async_variants::linear_object_pool::LinearObjectPool;
use crate::async_variants::page::Page;
use crate::page::PageId;
use futures::executor::block_on;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// Wrapper over T used by [`LinearObjectPool`].
///
/// Access is allowed with [`std::ops::Deref`] or [`std::ops::DerefMut`]
/// # Example
/// ```rust
///  use lockfree_object_pool::LinearObjectPool;
///  use std::sync::Arc;
///
///  let pool = Arc::new(LinearObjectPool::<u32>::new(
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
pub struct LinearOwnedReusable<
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    pool: Arc<LinearObjectPool<T, R, I>>,
    page_id: PageId,
    page: *const Page<T, I>,
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > LinearOwnedReusable<T, R, I>
{
    /// Create new element
    ///
    /// # Arguments
    /// * `pool` object pool owner
    /// * `page_id` page id
    /// * `page`    page that contains data
    /// # Safety
    /// * `page` has to be a valid pointer to a page in `pool`
    /// * `pool_id` has to be a valid id for `page`
    #[inline]
    pub(crate) unsafe fn new(
        pool: Arc<LinearObjectPool<T, R, I>>,
        page_id: PageId,
        page: &Page<T, I>,
    ) -> Self {
        Self {
            pool,
            page_id,
            page,
        }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > DerefMut for LinearOwnedReusable<T, R, I>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // SAFETY: there exists only this `LinearOwnedReusable` with this page_id
            self.page.as_ref().unwrap().get_mut(&self.page_id)
        }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Deref for LinearOwnedReusable<T, R, I>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            // SAFETY: there exists only this `LinearOwnedReusable` with this page_id
            self.page.as_ref().unwrap().get(&self.page_id)
        }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Drop for LinearOwnedReusable<T, R, I>
{
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // SAFETY: there exists only this `LinearOwnedReusable` with this page_id
            let page = self.page.as_ref().unwrap();
            block_on((self.pool.get_reset_callback())(
                page.get_mut(&self.page_id),
            ));
            page.free(&self.page_id);
        }
    }
}

unsafe impl<
        T: Send,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Send for LinearOwnedReusable<T, R, I>
{
} // SAFETY: sending the data is allowed if it's Send
unsafe impl<
        T: Send,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Sync for LinearOwnedReusable<T, R, I>
{
} // SAFETY: the Mutex manages synchronization so only Send is required
