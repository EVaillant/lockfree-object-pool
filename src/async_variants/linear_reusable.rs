use crate::async_variants::linear_object_pool::LinearObjectPool;
use crate::async_variants::page::PageId;
use crate::async_variants::page::Page;
use futures::executor::block_on;
use std::ops::{Deref, DerefMut};

/// Wrapper over T used by [`LinearObjectPool`].
///
/// Access is allowed with [`std::ops::Deref`] or [`std::ops::DerefMut`]
/// # Example
/// ```rust
///  use lockfree_object_pool::LinearObjectPool;
///
///  let pool = LinearObjectPool::<u32>::new(
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
pub struct LinearReusable<
    'a,
    T,
    R: AsyncFn(&mut T) + Clone + 'static,
    I: AsyncFn() -> T + Send + Sync + Clone + 'static,
> {
    pool: &'a LinearObjectPool<T, R, I>,
    page_id: PageId,
    page: &'a Page<T, I>,
}

impl<
        'a,
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > LinearReusable<'a, T, R, I>
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
        pool: &'a LinearObjectPool<T, R, I>,
        page_id: PageId,
        page: &'a Page<T, I>,
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
    > DerefMut for LinearReusable<'_, T, R, I>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // SAFETY: there exists only this `LinearReusable` with this page_id
            self.page.get_mut(&self.page_id)
        }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Deref for LinearReusable<'_, T, R, I>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe {
            // SAFETY: there exists only this `LinearReusable` with this page_id
            self.page.get(&self.page_id)
        }
    }
}

impl<
        T,
        R: AsyncFn(&mut T) + Clone + 'static,
        I: AsyncFn() -> T + Send + Sync + Clone + 'static,
    > Drop for LinearReusable<'_, T, R, I>
{
    #[inline]
    fn drop(&mut self) {
        let page = self.page;
        block_on((self.pool.get_reset_callback())(unsafe {
            // SAFETY: there exists only this `LinearReusable` with this page_id
            page.get_mut(&self.page_id)
        }));
        page.free(&self.page_id);
    }
}
