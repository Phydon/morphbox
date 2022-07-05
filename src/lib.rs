#[macro_use]
extern crate prettytable;

use chrono::Local;
use prettytable::{format, Cell, Row, Table};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;

use std::{
    collections::BTreeMap,
    cmp::min,
    fs,
    io::{self, prelude::*, BufReader, Result, Write},
};


// TODO ask user for filepath, than process it
const INPUT_FILE_PATH: &str = "./input/input_test.csv";

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

        Parameter { name, variations }
    }
}

pub fn input() -> String {
    println!("Enter \"Q\" when you`re done");
    println!("Enter a parameter: ");

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read input");

    return inp.trim().to_string();
}

pub fn input_variations() -> Vec<String> {
    let mut container: Vec<String> = Vec::new();

    loop {
        println!("Enter \"Q\" when you`re done");
        println!("Enter a variation: ");

        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read input");

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
    }

    parameters
}

pub fn read_input_file() -> Result<Vec<String>> {
    let file = fs::OpenOptions::new().read(true).open(INPUT_FILE_PATH)?;

    let reader = BufReader::new(file);
    let mut storage: Vec<String> = Vec::new();

    for line in reader.lines() {
        storage.push(line.unwrap());
    }

    Ok(storage)
}

// seperates the parameter from the variations
pub fn seperat_strings(storage: Vec<String>) -> Vec<Parameter> {
    let mut parameters: Vec<Parameter> = Vec::new();

    for item in storage {
        let mut temp_vec: Vec<&str> = item.split(",").collect();
        let parameter_name: String = temp_vec.remove(0).to_string();

        let mut variations: Vec<String> = Vec::new();

        for word in temp_vec {
            variations.push(word.to_string());
        }

        let param = Parameter::new(parameter_name, variations);
        parameters.push(param);
    }

    parameters
}

// process/transfrom input file
pub fn create_storage() -> Result<Vec<Parameter>> {
    let storage = read_input_file()?;
    let seperate_storage = seperat_strings(storage);

    Ok(seperate_storage)
}

pub fn create_container(parameters: &Vec<Parameter>) -> BTreeMap<&String, &Vec<String>> {
    let mut container: BTreeMap<_, _> = BTreeMap::new();

    for parameter in parameters {
        container.insert(&parameter.name, &parameter.variations);
    }

    container
}

pub fn create_table(container: BTreeMap<&String, &Vec<String>>) -> Table {
    if container.is_empty() {
        panic!("No arguments given");
    }

    let datetime = Local::now().to_string();
    let mut idx: i32 = 0;
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![Cell::new("MORPHBOX").style_spec("FrBdH3bc")]));
    table.add_row(Row::new(vec![Cell::new(&datetime).style_spec("FcH3ic")]));
    table.add_row(row![FdBwl->"INDEX", FdBwc->"PARAMETER", FdBwc->"VARIATIONS"]);

    for (key, values) in &container {
        let mut temp_str: String = String::new();
        for value in values.into_iter() {
            temp_str = temp_str + value + &" | ".to_string();
        }
        table.add_row(row![Fy->idx, b->key, c->temp_str]);
        idx += 1;
    }

    table
}

// async?
fn progress_bar(end: u64) -> ProgressBar {
    let pb = ProgressBar::new(end);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    pb
}

// create all possible combinations
// output can be ridiculously huge => LIMIT IT
// TODO let user filter out unrealistic combinations to reduce the output -> How?
pub fn combine(lst: Vec<Parameter>) -> Vec<String> {
    println!("Calculating combinations ...");

    let mut all_variations: Vec<Vec<String>> = Vec::new();

    for parameter in lst {
        let var = parameter.variations;
        all_variations.push(var);
    }

    let mut multi_prod = all_variations.into_iter().multi_cartesian_product();

    let len = multi_prod.clone().count() as u64;
    let pb = progress_bar(len);

    let mut comb_container: Vec<String> = Vec::new();
    let mut idx: u64 = 0;

    while let Some(var) = multi_prod.next() {
        // println!("{}: {:?}", idx, n);

        let new = min(idx, len);
        if idx % 10 == 0 {
            pb.set_position(new);
        }

        let var_str: String = var.join(",");

        comb_container.push(var_str);
        idx += 1;
    }

    pb.finish_with_message("done");

    comb_container
}

pub fn write_table_to_file(path: &str, table: &Table) -> io::Result<()> {
    let datetime = Local::now().to_string();
    let new_path = "./output/".to_string() + &datetime + "_" + path ;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_path)?;

    writeln!(file, "{}", table)?;

    Ok(())
}

// TODO can take a moment
// async?
// TODO sort the list by (index?)
pub fn write_combinations_to_file(path: &str, lst: &Vec<String>) -> io::Result<()> {
    let datetime = Local::now().to_string();
    let new_path = "./output/".to_string() + &datetime + "_" + path ;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_path)?;

    println!("Generating csv file ...");

    let len = lst.len() as u64;
    let pb = progress_bar(len);
    let mut idx: u64 = 0;

    for v in lst {
        writeln!(file, "{v}")?;

        let new = min(idx + 1, len);
        idx = new;
        if idx % 10 == 0 {
            pb.set_position(new);
        }
    }

    pb.finish_with_message("done");

    Ok(())
}

pub fn are_u_done() -> bool {
    loop {
        println!("Done?");
        println!("Press \"Y\" to quit or \"N\" to make changes!");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_uppercase().as_str() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {
                println!("Not valid");
            }
        }
    }
}
