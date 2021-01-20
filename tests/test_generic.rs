#[macro_export]
macro_rules! test_generic_01 {
    ($name:ident, $expression:expr) => {
        #[test]
        fn $name() {
            let pool = $expression;
            for _ in 0..2 {
                let mut v = pool.pull();
                assert_eq!(*v, 0);
                *v += 1;
            }
        }
    };
}

#[macro_export]
macro_rules! test_generic_02 {
    ($name:ident, $expression:expr) => {
        #[test]
        fn $name() {
            use std::mem::forget;
            let pool = $expression;

            let mut addrs = Vec::new();

            for _ in 0..10 {
                let mut v = pool.pull();
                assert_eq!(*v, 0);
                *v += 1;
                assert_eq!(*v, 1);
                let o = &mut *v;
                *o += 1;
                assert_eq!(*o, 2);
                let addr = format!("{:?}", o as *const u32);
                if !addrs.contains(&addr) {
                    addrs.push(addr);
                }
                forget(o);
                assert_eq!(*v, 2);
            }

            assert_eq!(addrs.len(), 1);
            for _ in 0..2 {
                let mut v = pool.pull();
                assert_eq!(*v, 0);
                *v += 1;
            }
        }
    };
}

#[macro_export]
macro_rules! test_generic_03 {
    ($name:ident, $expression:expr) => {
        #[test]
        fn $name() {
            let pool = $expression;

            let mut addrs = Vec::new();

            for _ in 0..10 {
                let mut v1 = pool.pull();
                let mut v2 = pool.pull();
                let addr1 = format!("{:?}", &mut *v1 as *const u32);
                let addr2 = format!("{:?}", &mut *v2 as *const u32);

                if !addrs.contains(&addr1) {
                    addrs.push(addr1);
                }

                if !addrs.contains(&addr2) {
                    addrs.push(addr2);
                }
            }

            assert_eq!(addrs.len(), 2);
        }
    };
}

#[macro_export]
macro_rules! test_generic_04 {
    ($name:ident, $expression:expr) => {
        #[test]
        fn $name() {
            use std::sync::mpsc;
            use std::sync::Arc;
            use std::thread;

            let pool = Arc::new($expression);

            let (tx, rx) = mpsc::channel();
            let mut children = Vec::new();

            for id in 0..5 {
                let thread_tx = tx.clone();
                let thread_pool = Arc::clone(&pool);

                let child = thread::spawn(move || {
                    let mut msg = thread_pool.pull();
                    *msg = id;
                    thread_tx.send(*msg).unwrap();
                });
                children.push(child);
            }

            let mut msgs = Vec::new();
            for _ in 0..5 {
                let msg = rx.recv().unwrap();
                if !msgs.contains(&msg) && msg < 5 {
                    msgs.push(msg);
                }
            }
            assert_eq!(msgs.len(), 5);

            for child in children {
                child.join().unwrap();
            }
        }
    };
}
