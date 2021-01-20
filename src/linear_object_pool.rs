use crate::linear_page::LinearPage;
use crate::linear_reusable::LinearReusable;

pub struct LinearObjectPool<T> {
    reset: Box<dyn Fn(&mut T)>,
    init: Box<dyn Fn() -> T>,
    head: LinearPage<T>,
}

impl<T> LinearObjectPool<T> {
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

    pub fn pull(&self) -> LinearReusable<T> {
        let (page, page_id) = self.head.alloc(&self.init);
        LinearReusable::new(self, page_id, page)
    }

    pub fn get_reset_callback(&self) -> &Box<dyn Fn(&mut T)> {
        &self.reset
    }
}

unsafe impl<T> Send for LinearObjectPool<T> {}
unsafe impl<T> Sync for LinearObjectPool<T> {}
