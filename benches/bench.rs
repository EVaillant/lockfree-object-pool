#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Instant;

use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

macro_rules! pull_ {
    ($pool:ident, 1) => {
        $pool.pull()
    };
    ($pool:ident, 2) => {
        $pool.pull(|| Vec::with_capacity(4096))
    };
    ($pool:ident, 3) => {
        $pool.create().unwrap()
    };
}

macro_rules! bench_alloc_impl_ {
    ($group:expr, $name:literal, $expression:expr, $pull_impl:tt) => {
        $group.bench_function($name, |b| {
            let pool = $expression;
            let mut items = Vec::new();
            b.iter(|| {
                items.push(pull_!(pool, $pull_impl));
            });
        });
    };
}

macro_rules! bench_free_impl_ {
    ($group:expr, $name:literal, $expression:expr, $pull_impl:tt) => {
        $group.bench_function($name, |b| {
            b.iter_custom(|iter| {
                let pool = $expression;
                let mut items = Vec::new();
                for _ in 0..iter {
                    items.push(pull_!(pool, $pull_impl));
                }
                let start = Instant::now();
                items.clear();
                start.elapsed()
            });
        });
    };
}

macro_rules! bench_alloc_mt_impl_ {
    ($group:expr, $name:literal, $expression:expr, $pull_impl:tt) => {
        $group.bench_function($name, |b| {
            b.iter_custom(|iter| {
                let pool = Arc::new($expression);
                let start_barrier = Arc::new(Barrier::new(6));
                let stop_barrier = Arc::new(Barrier::new(6));
                let mut children = Vec::new();
                for _ in 0..5 {
                    let pool = Arc::clone(&pool);
                    let start_barrier = Arc::clone(&start_barrier);
                    let stop_barrier = Arc::clone(&stop_barrier);
                    let child = thread::spawn(move || {
                        let mut items = Vec::with_capacity(iter as usize);
                        start_barrier.wait();
                        for _ in 0..iter {
                            items.push(pull_!(pool, $pull_impl));
                        }
                        stop_barrier.wait();
                    });
                    children.push(child);
                }

                start_barrier.wait();
                let start = Instant::now();
                stop_barrier.wait();
                let duration = start.elapsed() / 5;

                for child in children {
                    child.join().unwrap();
                }

                duration
            });
        });
    };
}

macro_rules! bench_free_mt_impl_ {
    ($group:expr, $name:literal, $expression:expr, $pull_impl:tt) => {
        $group.bench_function($name, |b| {
            b.iter_custom(|iter| {
                let pool = Arc::new($expression);
                let start_barrier = Arc::new(Barrier::new(6));
                let stop_barrier = Arc::new(Barrier::new(6));
                let mut children = Vec::new();
                for _ in 0..5 {
                    let pool = Arc::clone(&pool);
                    let start_barrier = Arc::clone(&start_barrier);
                    let stop_barrier = Arc::clone(&stop_barrier);
                    let child = thread::spawn(move || {
                        let mut items = Vec::with_capacity(iter as usize);
                        for _ in 0..iter {
                            items.push(pull_!(pool, $pull_impl));
                        }
                        start_barrier.wait();
                        items.clear();
                        stop_barrier.wait();
                    });
                    children.push(child);
                }

                start_barrier.wait();
                let start = Instant::now();
                stop_barrier.wait();
                let duration = start.elapsed() / 5;

                for child in children {
                    child.join().unwrap();
                }

                duration
            });
        });
    };
}

struct Vec4096 {
    _data: Vec<u8>,
}

impl Default for Vec4096 {
    fn default() -> Self {
        Self {
            _data: Vec::with_capacity(16 * 1024),
        }
    }
}

impl sharded_slab::Clear for Vec4096 {
    fn clear(&mut self) {}
}

fn bench_alloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation");
    bench_alloc_impl_!(
        group,
        "none object poll",
        lockfree_object_pool::NoneObjectPool::new(|| Vec::<u8>::with_capacity(16 * 1024)),
        1
    );
    bench_alloc_impl_!(
        group,
        "mutex object poll",
        lockfree_object_pool::MutexObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_alloc_impl_!(
        group,
        "spin_lock object poll",
        lockfree_object_pool::SpinLockObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_alloc_impl_!(
        group,
        "linear object poll",
        lockfree_object_pool::LinearObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_alloc_impl_!(
        group,
        "crate 'object-pool'",
        object_pool::Pool::<Vec<u8>>::new(32, || Vec::with_capacity(4096)),
        2
    );
    bench_alloc_impl_!(
        group,
        "crate 'sharded-slab'",
        sharded_slab::Pool::<Vec4096>::new(),
        3
    );
    group.finish();
}

fn bench_free(c: &mut Criterion) {
    let mut group = c.benchmark_group("free");
    bench_free_impl_!(
        group,
        "none object poll",
        lockfree_object_pool::NoneObjectPool::new(|| Vec::<u8>::with_capacity(16 * 1024)),
        1
    );
    bench_free_impl_!(
        group,
        "mutex object poll",
        lockfree_object_pool::MutexObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_free_impl_!(
        group,
        "spin_lock object poll",
        lockfree_object_pool::SpinLockObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_free_impl_!(
        group,
        "linear object poll",
        lockfree_object_pool::LinearObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_free_impl_!(
        group,
        "crate 'object-pool'",
        object_pool::Pool::<Vec<u8>>::new(32, || Vec::with_capacity(4096)),
        2
    );
    bench_free_impl_!(
        group,
        "crate 'sharded-slab'",
        sharded_slab::Pool::<Vec4096>::new(),
        3
    );
    group.finish();
}

fn bench_alloc_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi thread allocation");
    bench_alloc_mt_impl_!(
        group,
        "none object poll",
        lockfree_object_pool::NoneObjectPool::new(|| Vec::<u8>::with_capacity(16 * 1024)),
        1
    );
    bench_alloc_mt_impl_!(
        group,
        "mutex object poll",
        lockfree_object_pool::MutexObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_alloc_mt_impl_!(
        group,
        "spin_lock object poll",
        lockfree_object_pool::SpinLockObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_alloc_mt_impl_!(
        group,
        "linear object poll",
        lockfree_object_pool::LinearObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_alloc_mt_impl_!(
        group,
        "crate 'object-pool'",
        object_pool::Pool::<Vec<u8>>::new(32, || Vec::with_capacity(4096)),
        2
    );
    bench_alloc_mt_impl_!(
        group,
        "crate 'sharded-slab'",
        sharded_slab::Pool::<Vec4096>::new(),
        3
    );
    group.finish();
}

fn bench_free_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi thread free");
    bench_free_mt_impl_!(
        group,
        "none object poll",
        lockfree_object_pool::NoneObjectPool::new(|| Vec::<u8>::with_capacity(16 * 1024)),
        1
    );
    bench_free_mt_impl_!(
        group,
        "mutex object poll",
        lockfree_object_pool::MutexObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_free_mt_impl_!(
        group,
        "spin_lock object poll",
        lockfree_object_pool::SpinLockObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_free_mt_impl_!(
        group,
        "linear object poll",
        lockfree_object_pool::LinearObjectPool::<Vec<u8>>::new(
            || Vec::with_capacity(16 * 1024),
            |_v| {}
        ),
        1
    );
    bench_free_mt_impl_!(
        group,
        "crate 'object-pool'",
        object_pool::Pool::<Vec<u8>>::new(32, || Vec::with_capacity(4096)),
        2
    );
    bench_free_mt_impl_!(
        group,
        "crate 'sharded-slab'",
        sharded_slab::Pool::<Vec4096>::new(),
        3
    );
    group.finish();
}

criterion_group!(multi_thread, bench_alloc_mt, bench_free_mt);
criterion_group!(mono_thread, bench_alloc, bench_free);
criterion_main!(mono_thread, multi_thread);
