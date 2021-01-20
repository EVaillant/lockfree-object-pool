#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::time::Instant;

use std::sync::Arc;
use std::sync::Barrier;
use std::thread;

use lockfree_object_pool::LinearObjectPool;
use lockfree_object_pool::SpinLockObjectPool;
use lockfree_object_pool::MutexObjectPool;
use lockfree_object_pool::NoneObjectPool;

macro_rules! bench_alloc_impl_ {
    ($group:expr, $name: literal, $expression:expr) => {
        $group.bench_function(format!("{} object poll", $name), |b| {
            let pool = $expression;
            let mut items = Vec::new();
            b.iter(|| {
                items.push(pool.pull());
            });
        });
    };
}

macro_rules! bench_free_impl_ {
    ($group:expr, $name: literal, $expression:expr) => {
        $group.bench_function(format!("{} object poll", $name), |b| {
            b.iter_custom(|iter| {
                let pool = $expression;
                let mut items = Vec::new();
                for _ in 0..iter {
                    items.push(pool.pull());
                }

                let start = Instant::now();
                items.clear();
                start.elapsed()
            });
        });
    };
}

macro_rules! bench_alloc_mt_impl_ {
    ($group:expr, $name: literal, $expression:expr) => {
        $group.bench_function(format!("{} object poll", $name), |b| {
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
                        let mut items = Vec::new();
                        start_barrier.wait();
                        for _ in 0..iter {
                            items.push(pool.pull());
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
    ($group:expr, $name: literal, $expression:expr) => {
        $group.bench_function(format!("{} object poll", $name), |b| {
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
                        let mut items = Vec::new();
                        for _ in 0..iter {
                            items.push(pool.pull());
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

fn bench_alloc(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation");
    bench_alloc_impl_!(
        group,
        "none",
        NoneObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024))
    );
    bench_alloc_impl_!(
        group,
        "mutex",
        MutexObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_alloc_impl_!(
        group,
        "spin_lock",
        SpinLockObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_alloc_impl_!(
        group,
        "linear",
        LinearObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    group.finish();
}

fn bench_free(c: &mut Criterion) {
    let mut group = c.benchmark_group("free");
    bench_free_impl_!(
        group,
        "none",
        NoneObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024))
    );
    bench_free_impl_!(
        group,
        "mutex",
        MutexObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_free_impl_!(
        group,
        "spin_lock",
        SpinLockObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_free_impl_!(
        group,
        "linear",
        LinearObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    group.finish();
}

fn bench_alloc_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi thread allocation");
    bench_alloc_mt_impl_!(
        group,
        "none",
        NoneObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024))
    );
    bench_alloc_mt_impl_!(
        group,
        "mutex",
        MutexObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_alloc_mt_impl_!(
        group,
        "spin_lock",
        SpinLockObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_alloc_mt_impl_!(
        group,
        "linear",
        LinearObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    group.finish();
}

fn bench_free_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi thread free");
    bench_free_mt_impl_!(
        group,
        "none",
        NoneObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024))
    );
    bench_free_mt_impl_!(
        group,
        "mutex",
        MutexObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_free_mt_impl_!(
        group,
        "spin_lock",
        SpinLockObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    bench_free_mt_impl_!(
        group,
        "linear",
        LinearObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), |_v| {})
    );
    group.finish();
}

criterion_group!(multi_thread, bench_alloc_mt, bench_free_mt);
criterion_group!(mono_thread, bench_alloc, bench_free);
criterion_main!(mono_thread, multi_thread);
