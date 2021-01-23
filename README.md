# Lock Free Object Pool
[![License](https://img.shields.io/badge/License-Boost%201.0-lightblue.svg)](https://github.com/EVaillant/lockfree-object-pool) [![Cargo](https://img.shields.io/crates/v/lockfree-object-pool.svg)](https://crates.io/crates/lockfree-object-pool) [![Documentation](https://docs.rs/lockfree-object-pool/badge.svg)](
https://docs.rs/lockfree-object-pool) ![CI](https://github.com/EVaillant/lockfree-object-pool/workflows/CI/badge.svg)

A thread-safe object pool collection with automatic return.

Some implementations are lockfree :
* LinearObjectPool
* SpinLockObjectPool

Other use std::Mutex :
* MutexObjectPool

And NoneObjectPool basic allocation without pool.

### Usage
```toml
[dependencies]
lockfree-object-pool = "0.1"
```
```rust
extern crate lockfree_object_pool;
```

### Example

The general pool creation looks like this for
```rust
 let pool = LinearObjectPool::<u32>::new(
     ||  Default::default(), 
     |v| {*v = 0; });
```

And use the object pool 
```rust
  let mut item = pool.pull();
  *item = 5;
  ...  
```
At the end of the scope item return in object pool.

### Interface
All implementations support same interface :
```rust
struct ObjectPool<T> {  
}

impl<T> ObjectPool<T> {
  // for LinearObjectPool, SpinLockObjectPool and MutexObjectPool
  // init closure used to create an element
  // reset closure used to reset element a dropped element
  pub fn new<R, I>(init: I, reset: R) -> Self
    where
        R: Fn(&mut T) + 'static,
        I: Fn() -> T + 'static + Clone,
    {
      ...
    }

  // for NoneObjectPool
  // init closure used to create an element
  pub fn new<I>(init: I) -> Self
    where
        I: Fn() -> T + 'static
    {
      ...
    }

  pub fn pull(&self) -> Reusable<T> {
    ...
  }
}

struct Reusable<T> {  
}

impl<'a, T> DerefMut for Reusable<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        ...
    }
}

impl<'a, T> Deref for MutexReusable<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        ...
    }
}
```

### Multithreading

All implementation support allocation/desallocation from on or more thread. You only need to wrap the pool in a [`std::sync::Arc`] :

```rust
 let pool = Arc::new(LinearObjectPool::<u32>::new(
     ||  Default::default(), 
     |v| {*v = 0; }));
```

### Performance

Global [report](https://evaillant.github.io/lockfree-object-pool/benches/criterion/report/index.html).

#### Allocation

 ObjectPool | Duration in Monothreading (us) | Duration Multithreading (us)
------------| ------------------------------ |-----------------------------
NoneObjectPool|1.2162|0.63033
MutexObjectPool|1.2458|1.5140
SpinLockObjectPool|1.2437|1.3737
LinearObjectPool|0.21764|0.22418
[`crate 'sharded-slab'`]|1.5|0.82790
[`crate 'object-pool'`]|0.61956|0.26323

Report [monothreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/allocation/report/index.html) and [multithreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/multi%20thread%20allocation/report/index.html).

#### Desallocation

ObjectPool | Duration in Monothreading (ns) | Duration Multithreading (ns)
-----------| ------------------------------ |-----------------------------
NoneObjectPool|91.362|86.530
MutexObjectPool|25.486|101.40
SpinLockObjectPool|22.089|50.411
LinearObjectPool|7.1384|34.481
[`crate 'sharded-slab'`]|9.0273|11.127
[`crate 'object-pool'`]|20.038|47.768

Report [monothreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/free/report/index.html) and [multithreading](https://evaillant.github.io/lockfree-object-pool/benches/criterion/multi%20thread%20free/report/index.html).

### Comparison with Similar Crates

* [`crate 'sharded-slab'`]: I like pull interface but i dislike 
  * Default / Reset trait because not enough flexible
  * Performance

* [`crate 'object-pool'`]: use a spinlock to sync and the performance are pretty good but i dislike :
  * need to specify fallback at each pull call :
  ```rust
  use object_pool::Pool;
  let pool = Pool::<Vec<u8>>::new(32, || Vec::with_capacity(4096);
  // ...
  let item1 = pool.pull(|| Vec::with_capacity(4096));
  // ...
  let item2 = pool.pull(|| Vec::with_capacity(4096));
  ```
  * no reset mechanism, need to do manually

### TODO

* why the object-pool with spinlock has so bad performance compared to spinlock mutex use by [`crate 'object-pool'`]
* have a Poll::create_owned like in [`crate 'sharded-slab'`]
* impl a tree object pool (cf [`toolsbox`])

### Implementation detail

TODO

### Licence

cf [Boost Licence](http://www.boost.org/LICENSE_1_0.txt)

### Related Projects

- [`crate 'object-pool'`] - A thread-safe object pool in rust with mutex 
- [`crate 'sharded-slab'`] - A lock-free concurrent slab
- [`toolsbox`] - Some object pool implementation en c++


[`crate 'sharded-slab'`]: https://crates.io/crates/sharded-slab
[`crate 'object-pool'`]: https://crates.io/crates/object-pool
[`toolsbox`]: https://github.com/EVaillant/toolsbox
