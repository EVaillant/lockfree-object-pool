use lockfree_object_pool::MutexObjectPool;

#[macro_use]
mod test_generic;

fn make_pool() -> MutexObjectPool<u32> {
    MutexObjectPool::<u32>::new(Default::default, |v| {
        *v = 0;
    })
}

test_generic_01!(test_mutex_01, make_pool());
test_generic_02!(test_mutex_02, make_pool());
test_generic_03!(test_mutex_03, make_pool());
test_generic_04!(test_mutex_04, make_pool());
