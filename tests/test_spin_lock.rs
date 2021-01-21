use lockfree_object_pool::SpinLockObjectPool;

#[macro_use]
mod test_generic;

fn make_pool() -> SpinLockObjectPool<u32> {
    SpinLockObjectPool::<u32>::new(Default::default, |v| {
        *v = 0;
    })
}

test_generic_01!(test_spin_lock_01, make_pool());
test_generic_02!(test_spin_lock_02, make_pool());
test_generic_03!(test_spin_lock_03, make_pool());
test_generic_04!(test_spin_lock_04, make_pool());
