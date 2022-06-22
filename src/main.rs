use morphbox::*;

fn main() {
    let args: Vec<String> = vec!["Some".to_string(), "Thing".to_string()];

    let param = Parameter::new("NewName".to_string(), args);

    println!("param: {:?}", param);
    println!("name: {:?}", param.name);
    println!("variations: {:?}", param.variations);
}
