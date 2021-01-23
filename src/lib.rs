//! A thread-safe object pool collection with automatic return.
//!
//! Some implementations are lockfree :
//! * [`LinearObjectPool`]
//! * [`SpinLockObjectPool`]
//!
//! Other use std::Mutex :
//! * [`MutexObjectPool`]
//!
//! And [`NoneObjectPool`] basic allocation without pool.
//!
//! ## Example
//!
//! The general pool creation looks like this for
//! ```rust
//!   use lockfree_object_pool::LinearObjectPool;
//!   
//!   let pool = LinearObjectPool::<u32>::new(
//!     ||  Default::default(),
//!     |v| {*v = 0; });
//!
//!   // And use the object pool
//!   let mut item = pool.pull();
//!   *item = 5;
//! ```
//! At the end of the scope item return in object pool.
//! ## Multithreading
//!
//! All implementation support allocation/desallocation from on or more thread. You only need to wrap the pool in a [`std::sync::Arc`] :
//!
//! ```rust
//!   use lockfree_object_pool::LinearObjectPool;
//!   use std::sync::Arc;
//!
//!   let pool = Arc::new(LinearObjectPool::<u32>::new(
//!        ||  Default::default(),
//!        |v| {*v = 0; }));
//! ```
//! ## Performance
//!
//! Global [report](https://evaillant.github.io/lockfree-object-pool/benches/criterion/report/index.html).
//!
//! ### Allocation
//!
//! ObjectPool | Duration in Monothreading (us) | Duration Multithreading (us)
//! -----------| ------------------------------ |-----------------------------
//! [`NoneObjectPool`]|1.2162|0.63033
//! [`MutexObjectPool`]|1.2458|1.5140
//! [`SpinLockObjectPool`]|1.2437|1.3737
//! [`LinearObjectPool`]|0.21764|0.22418
//! [`crate 'sharded-slab'`]|1.5|0.82790
//! [`crate 'object-pool'`]|0.61956|0.26323
//!
//! Report [monothreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/allocation/report/index.html) and [multithreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/multi%20thread%20allocation/report/index.html).
//!
//! ### Desallocation
//!
//! ObjectPool | Duration in Monothreading (ns) | Duration Multithreading (ns)
//! -----------| ------------------------------ |-----------------------------
//! [`NoneObjectPool`]|91.362|86.530
//! [`MutexObjectPool`]|25.486|101.40
//! [`SpinLockObjectPool`]|22.089|50.411
//! [`LinearObjectPool`]|7.1384|34.481
//! [`crate 'sharded-slab'`]|9.0273|11.127
//! [`crate 'object-pool'`]|20.038|47.768
//!
//! [`crate 'sharded-slab'`]: https://crates.io/crates/sharded-slab
//! [`crate 'object-pool'`]: https://crates.io/crates/object-pool
//!
//! Report [monothreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/free/report/index.html) and [multithreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/multi%20thread%20free/report/index.html).
mod linear_object_pool;
mod linear_page;
mod linear_reusable;
mod mutex_object_pool;
mod mutex_reusable;
mod none_object_pool;
mod none_reusable;
mod page;
mod spin_lock;
mod spin_lock_object_pool;
mod spin_lock_reusable;

pub use linear_object_pool::LinearObjectPool;
pub use linear_reusable::LinearReusable;
pub use mutex_object_pool::MutexObjectPool;
pub use mutex_reusable::MutexReusable;
pub use none_object_pool::NoneObjectPool;
pub use none_reusable::NoneReusable;
pub use spin_lock_object_pool::SpinLockObjectPool;
pub use spin_lock_reusable::SpinLockReusable;
