use template;

#[test]
fn test_test() {
    let indeed: bool = true;
    assert!(indeed);
}

#[test]
#[should_panic(expected = "panic msg")]
fn panic_test() {
    panic!("panic msg");
}
