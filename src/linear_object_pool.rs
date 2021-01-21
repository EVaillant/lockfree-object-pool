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
    reset: Box<dyn Fn(&mut T)>,
    init: Box<dyn Fn() -> T>,
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
        R: Fn(&mut T) + 'static,
        I: Fn() -> T + 'static + Clone,
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
        LinearReusable::new(self, page_id, page)
    }

    #[doc(hidden)]
    pub fn get_reset_callback(&self) -> &Box<dyn Fn(&mut T)> {
        &self.reset
    }
}

unsafe impl<T> Send for LinearObjectPool<T> {}
unsafe impl<T> Sync for LinearObjectPool<T> {}
