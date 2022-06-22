use morphbox::*;

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

#[test]
fn new_parameters_enough_parameters_test() {
    let v: Vec<String> = vec!["Some".to_string(), "Thing".to_string()];
    let param = Parameter::new("MyName".to_string(), v);

    assert_eq!(param.name, "MyName".to_string());
    assert_eq!(param.variations, ["Some".to_string(), "Thing".to_string()]);
}

#[test]
#[should_panic(expected = "Not enough arguments")]
fn new_parameters_not_enough_parameters_test() {
    let v: Vec<String> = vec![];
    let param = Parameter::new("MyName".to_string(), v);

    assert_eq!(param.name, "MyName".to_string());
    assert_eq!(param.variations, ["Some".to_string(), "Thing".to_string()]);
}
