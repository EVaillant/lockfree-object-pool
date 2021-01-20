use crate::linear_object_pool::LinearObjectPool;
use crate::page::{Page, PageId};
use std::ops::{Deref, DerefMut};

pub struct LinearReusable<'a, T> {
    pool: &'a LinearObjectPool<T>,
    page_id: PageId,
    page: *const Page<T>,
}

impl<'a, T> LinearReusable<'a, T> {
    pub fn new(pool: &'a LinearObjectPool<T>, page_id: PageId, page: *const Page<T>) -> Self {
        Self {
            pool,
            page_id,
            page,
        }
    }

    fn get_page(&self) -> &Page<T> {
        unsafe { self.page.as_ref().unwrap() }
    }

    fn get_mut_page(&self) -> &mut Page<T> {
        unsafe { (self.page as *mut Page<T>).as_mut().unwrap() }
    }
}

impl<'a, T> DerefMut for LinearReusable<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut_page().get_mut(&self.page_id)
    }
}

impl<'a, T> Deref for LinearReusable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_page().get(&self.page_id)
    }
}

impl<'a, T> Drop for LinearReusable<'a, T> {
    fn drop(&mut self) {
        let page = self.get_mut_page();
        (self.pool.get_reset_callback())(page.get_mut(&self.page_id));
        page.free(&self.page_id);
    }
}
