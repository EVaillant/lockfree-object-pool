use std::sync::atomic::{AtomicU32, Ordering};

pub struct Page<T> {
    data: [T; 32],
    free: AtomicU32,
}

pub type PageId = u8;

impl<T> Page<T> {
    pub fn new<I>(init: I) -> Self
    where
        I: Fn() -> T,
    {
        Self {
            data: [
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
                init(),
            ],
            free: AtomicU32::new(u32::MAX),
        }
    }

    pub fn is_full(&self) -> bool {
        self.free.load(Ordering::Relaxed) == 0
    }

    pub fn get_mask(&self) -> u32 {
        self.free.load(Ordering::Relaxed)
    }

    pub fn alloc(&self) -> Option<PageId> {
        for i in 0..32 {
            let mask: u32 = 1 << i;
            let mut old = self.free.load(Ordering::Relaxed);
            loop {
                if old == 0 {
                    return None;
                }
                if old & mask == 0 {
                    break;
                }
                let new = old & !mask;
                match self
                    .free
                    .compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed)
                {
                    Ok(_) => return Some(i),
                    Err(x) => old = x,
                }
            }
        }
        None
    }

    pub fn free(&self, id: &PageId) {
        let mask: u32 = 1 << id;
        let mut old = self.free.load(Ordering::Relaxed);
        loop {
            let new = old | mask;
            match self
                .free
                .compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(x) => old = x,
            }
        }
    }

    pub fn get(&self, id: &PageId) -> &T {
        &self.data[*id as usize]
    }

    pub fn get_mut(&mut self, id: &PageId) -> &mut T {
        &mut self.data[*id as usize]
    }
}
