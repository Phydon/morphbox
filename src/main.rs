use std::process;

use morphbox::*;

fn main() {
    let args: Vec<String> = vec!["Some".to_string(), "Thing".to_string()];

    let param = Parameter::new("NewName".to_string(), args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("param: {:?}", param);
    println!("name: {:?}", param.name);
    println!("variations: {:?}", param.variations);
}
