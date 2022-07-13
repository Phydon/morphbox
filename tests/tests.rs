use std::collections::BTreeMap;

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

#[test]
#[should_panic(expected = "No arguments given")]
fn create_table_panic_test() {
    let empty_storage: BTreeMap<_,_> = BTreeMap::new();
    let _empty_table = create_table(empty_storage);
}

#[test]
fn combine_test() {
    let param_a = Parameter::new("A".to_string(), vec!["aa".to_string(), "aaa".to_string()]);
    let param_b = Parameter::new("B".to_string(), vec!["bb".to_string(), "bbb".to_string()]);
    let mut vec_par: Vec<Parameter> = Vec::new();
    vec_par.push(param_a);
    vec_par.push(param_b);

    let comb: Vec<String> = combine(&vec_par);
    // println!("{:?}", comb);

    assert_eq!(comb, ["aa,bb", "aa,bbb", "aaa,bb", "aaa,bbb"]);
}
