#[macro_use] 
extern crate prettytable;

use prettytable::{Table, Row, Cell, format};
use chrono::Local;

use std::{
    io::{self, Write},
    fs,
    collections::BTreeMap,
};

const FILEPATH: &str = "./mymorphbox.txt";

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

pub fn cycle_inputs() -> Vec<Parameter> {
    let mut parameters: Vec<Parameter> = Vec::new();

    loop {
        let parameter_name: String = input();
        match parameter_name.as_str() {
            "q" | "Q" => break,
            _ => (),
        };

        let variations: Vec<String> = input_variations();

        let param = Parameter::new(parameter_name, variations);
        parameters.push(param);
    };

    parameters
}

pub fn create_table(container: BTreeMap<String, Vec<String>>) -> Table {
    let datetime = Local::now().to_string();
    let mut idx: i32 = 0;
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
            Cell::new("MORPHBOX")
                .style_spec("FrBdH3bc")]));
    table.add_row(Row::new(vec![
            Cell::new(&datetime)
                .style_spec("FcH3ic")]));
    table.add_row(row![FdBwl->"INDEX", FdBwc->"PARAMETER", FdBwc->"VARIATIONS"]);

    for (key, values) in &container {
        let mut temp_str: String = String::new();
        for value in values {
            temp_str = temp_str + value + &" | ".to_string(); 
        }
        table.add_row(row![Fy->idx, b->key, c->temp_str]);
        idx += 1;
    }

    table
}

pub fn create_container(parameters: Vec<Parameter>) -> BTreeMap<String, Vec<String>> {
    let mut container: BTreeMap<_,_> = BTreeMap::new();

    for parameter in parameters {
        container.insert(parameter.name, parameter.variations);
    }

    container
}

pub fn write_to_file(content: &Table) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILEPATH)?;

    writeln!(file, "{}", content)?;
    Ok(())
    }

pub fn are_u_done(table: &Table) -> bool {
    loop {
        table.printstd();

        println!("Done?");
        println!("Press \"Y\" to quit or \"N\" to make changes!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().to_uppercase().as_str() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {
                println!("Not valid");
            },
        }
    }
}
