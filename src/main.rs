use std::process;

use morphbox::*;

fn main() {
    println!("Hello World!");
    let conf = Config::new("Filename".to_string()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    morphbox::run(conf);
}
