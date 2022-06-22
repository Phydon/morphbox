use std::process;

use template::*;

fn main() {
    println!("Hello World!");
    let conf = Config::new("Filename".to_string()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    template::run(conf);
}
