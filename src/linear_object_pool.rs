use crate::linear_page::LinearPage;
use crate::linear_reusable::LinearReusable;

/// ObjectPool use a lockfree vector to secure multithread access to pull.
///
/// The lockfree vector is implemented as linked list.
///
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
pub struct LinearObjectPool<T> {
    reset: Box<dyn Fn(&mut T) + Send + Sync>,
    init: Box<dyn Fn() -> T + Send + Sync>,
    head: LinearPage<T>,
}

impl<T> LinearObjectPool<T> {
    ///
    /// Create an new [`LinearObjectPool`]
    ///
    /// # Arguments
    /// * `init`  closure to create new item
    /// * `reset` closure to reset item before reusage
    ///
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
    /// ```
    pub fn new<R, I>(init: I, reset: R) -> Self
    where
        R: Fn(&mut T) + 'static + Send + Sync,
        I: Fn() -> T + 'static + Clone + Send + Sync,
    {
        Self {
            reset: Box::new(reset),
            init: Box::new(init.clone()),
            head: LinearPage::new(init),
        }
    }

    ///
    /// Create a new element. When the element is dropped, it returns in the pull.
    ///
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
    /// ```
    pub fn pull(&self) -> LinearReusable<T> {
        let (page, page_id) = self.head.alloc(&self.init);
        unsafe { LinearReusable::new(self, page_id, page) }
    }

    pub(crate) fn get_reset_callback(&self) -> &dyn Fn(&mut T) {
        &self.reset
    }
}
