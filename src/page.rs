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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_page_01() {
        let page = Page::<u32>::new(|| 0);
        assert_eq!(page.is_full(), false);
        assert_eq!(page.get_mask(), u32::MAX);
    }

    #[test]
    fn test_page_02() {
        let page = Page::<u32>::new(|| 0);

        let item1 = page.alloc();
        assert_eq!(item1.is_none(), false);
        assert_eq!(item1.unwrap(), 0);
        assert_eq!(
            format!("{:b}", page.get_mask()),
            "11111111111111111111111111111110"
        );

        let item2 = page.alloc();
        assert_eq!(item2.is_none(), false);
        assert_eq!(item2.unwrap(), 1);
        assert_eq!(
            format!("{:b}", page.get_mask()),
            "11111111111111111111111111111100"
        );

        let item3 = page.alloc();
        assert_eq!(item3.is_none(), false);
        assert_eq!(item3.unwrap(), 2);
        assert_eq!(
            format!("{:b}", page.get_mask()),
            "11111111111111111111111111111000"
        );

        page.free(&item2.unwrap());
        assert_eq!(
            format!("{:b}", page.get_mask()),
            "11111111111111111111111111111010"
        );

        page.free(&item1.unwrap());
        assert_eq!(
            format!("{:b}", page.get_mask()),
            "11111111111111111111111111111011"
        );

        page.free(&item3.unwrap());
        assert_eq!(
            format!("{:b}", page.get_mask()),
            "11111111111111111111111111111111"
        );
    }

    #[test]
    fn test_page_03() {
        let page = Page::<u32>::new(|| 0);
        for i in 0..32 {
            assert_eq!(page.is_full(), false);

            let item = page.alloc();
            assert_eq!(item.is_none(), false);
            assert_eq!(item.unwrap(), i);
        }
        assert_eq!(page.is_full(), true);
        let item = page.alloc();
        assert_eq!(item.is_none(), true);
    }
}
