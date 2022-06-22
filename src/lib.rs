use std::{
    io,
    collections::BTreeMap,
};

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub variations: Vec<String>,
}

impl Parameter {
    pub fn new(word: String, var: Vec<String>) -> Parameter {
        if var.len() <= 0 {
            panic!("Not enough arguments")
        }
        let name = word;
        let variations = var;

        Parameter {name, variations}
    }
}

pub fn input() -> String {
        println!("Enter \"Q\" when you`re done");
        println!("Enter a parameter: ");

        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input");

        return inp.trim().to_string()
}

pub fn input_variations() -> Vec<String> {
    let mut container: Vec<String> = Vec::new();
    loop {
        println!("Enter \"Q\" when you`re done");
        println!("Enter a variation: ");

        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("Failed to read input");

        match inp.trim() {
            "q" | "Q" => return container,
            _ => container.push(inp.trim().to_string()),
        }
    }
}


