# Lock Free Object Pool
[![License](https://img.shields.io/badge/License-Boost%201.0-lightblue.svg)](https://github.com/EVaillant/lockfree-object-pool)

A thread-safe object pool collection with automatic return and attach/detach semantics.

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

Global [report](./benches/report/index.html).

#### Allocation

 ObjectPool | Duration in Monothreading (us) | Duration Multithreading (us)
------------| ------------------------- |------------------------
NoneObjectPool|1.2937|587.75
MutexObjectPool|1.3143|1.3210
SpinLockObjectPool|1.3170|1.2555
LinearObjectPool|0.29399|0.19894

Report [monothreading](./benches/criterion/allocation/report/index.html) and [multithreading](./benches/criterion/multi%20thread%20allocation/report/index.html).

#### Desallocation

ObjectPool | Duration in Monothreading (ns) | Duration Multithreading (ns)
------------| ------------------------- |------------------------
NoneObjectPool|114.22|25.474
MutexObjectPool|26.173|99.511
SpinLockObjectPool|22.490|52.378
LinearObjectPool|9.9155|23.028

Report [monothreading](./benches/criterion/free/report/index.html) and [multithreading](./benches/criterion/multi%20thread%20free/report/index.html).


### Implementation detail

TODO

### Licence

cf [Boost Licence](http://www.boost.org/LICENSE_1_0.txt)

### Related Projects

- [object-pool](https://github.com/CJP10/object-pool) - A thread-safe object pool in rust with mutex 
- [toolsbox](https://github.com/EVaillant/toolsbox) - Some object pool implementation en c++