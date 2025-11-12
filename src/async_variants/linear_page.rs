use crate::page::PageId;
use crate::async_variants::page::Page;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct LinearPage<T, I: AsyncFn() -> T + Send + Sync + Clone + 'static> {
    page: Page<T, I>,
    next: AtomicPtr<LinearPage<T, I>>,
}

impl<T, I: AsyncFn() -> T + Send + Sync + Clone + 'static> LinearPage<T, I> {
    #[inline]
    pub async fn new(init: I) -> Self {
        Self {
            page: Page::new(init).await,
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }

    #[inline]
    pub async fn get_or_create_next(&self, init: I) -> &Self {
        let mut current = self.next.load(Ordering::Relaxed);
        if current.is_null() {
            let new = Box::into_raw(Box::new(LinearPage::new(init).await));
            match self
                .next
                .compare_exchange(current, new, Ordering::SeqCst, Ordering::Relaxed)
            {
                Ok(_) => {
                    current = new;
                }
                Err(x) => {
                    unsafe {
                        // SAFETY: new was been allocated by Box::new
                        drop(Box::from_raw(new))
                    };
                    current = x;
                }
            }
        }
        unsafe {
            // SAFETY: there are no mutable references to current
            current.as_ref().unwrap()
        }
    }

    #[inline]
    pub async fn alloc(&self, init: I) -> (&Page<T, I>, PageId) {
        let mut linear_page = self;
        loop {
            match linear_page.page.alloc() {
                Some(id) => {
                    return (&linear_page.page, id);
                }
                None => {
                    linear_page = linear_page.get_or_create_next(init.clone()).await;
                }
            };
        }
    }
}

impl<T, I: AsyncFn() -> T + Send + Sync + Clone + 'static> Drop for LinearPage<T, I> {
    #[inline]
    fn drop(&mut self) {
        let current = self.next.load(Ordering::Relaxed);
        if !current.is_null() {
            unsafe {
                // SAFETY: current was allocated with Box::new
                drop(Box::from_raw(current))
            };
        }
    }
}
