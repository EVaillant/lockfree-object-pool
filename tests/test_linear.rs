use lockfree_object_pool::LinearObjectPool;

#[macro_use]
mod test_generic;

fn make_pool() -> LinearObjectPool<u32> {
    LinearObjectPool::<u32>::new(
        || Default::default(),
        |v| {
            *v = 0;
        },
    )
}

test_generic_01!(test_linear_01, make_pool());
test_generic_02!(test_linear_02, make_pool());
test_generic_03!(test_linear_03, make_pool());
test_generic_04!(test_linear_04, make_pool());

#[test]
fn test_linear_05() {
    let pool = LinearObjectPool::<Vec<u8>>::new(|| Vec::with_capacity(16 * 1024), move |_v| {});
    let mut items = Vec::new();

    for _ in 0..50 {
        items.push(pool.pull());
    }
}
