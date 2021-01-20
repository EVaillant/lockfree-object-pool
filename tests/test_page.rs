use lockfree_object_pool::Page;

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
