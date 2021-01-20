use crate::page::{Page, PageId};
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct LinearPage<T> {
    page: Page<T>,
    next: AtomicPtr<LinearPage<T>>,
}

impl<T> LinearPage<T> {
    pub fn new<I>(init: I) -> Self
    where
        I: Fn() -> T,
    {
        Self {
            page: Page::new(init),
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn get_or_create_next<I>(&self, init: I) -> &Self
    where
        I: Fn() -> T,
    {
        let mut current = self.next.load(Ordering::Relaxed);
        if current.is_null() {
            let new = Box::into_raw(Box::new(LinearPage::<T>::new(init)));
            match self
                .next
                .compare_exchange_weak(current, new, Ordering::SeqCst, Ordering::Relaxed)
            {
                Ok(_) => {
                    current = new;
                }
                Err(x) => {
                    unsafe { Box::from_raw(new) };
                    current = x;
                }
            }
        }
        unsafe { current.as_ref().unwrap() }
    }

    pub fn alloc<I>(&self, init: I) -> (*const Page<T>, PageId)
    where
        I: Fn() -> T + Clone,
    {
        let mut linear_page = self;
        loop {
            match linear_page.page.alloc() {
                Some(id) => {
                    return (&linear_page.page, id);
                }
                None => {
                    linear_page = linear_page.get_or_create_next(init.clone());
                }
            };
        }
    }
}

impl<T> Drop for LinearPage<T> {
    fn drop(&mut self) {
        let current = self.next.load(Ordering::Relaxed);
        if !current.is_null() {
            unsafe { Box::from_raw(current) };
        }
    }
}
