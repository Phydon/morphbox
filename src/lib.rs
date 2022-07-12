#[macro_use]
extern crate prettytable;

use chrono::Local;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use prettytable::{format, Cell, Row, Table};
use rand::Rng;
use terminal_size::{terminal_size, Height, Width};

use std::{
    cmp::min,
    collections::BTreeMap,
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
    println!("{}", "\nEnter \"Q\" when you`re done".dimmed());
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
        println!("{}", "\nEnter \"Q\" when you`re done".dimmed());
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

pub fn ask_for_file() -> bool {
    loop {
        println!("\nRead data from a file?");
        println!("      [ F ]     => use FILE");
        println!("      [ M ]     => enter data MANUALLY");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "f" | "F" => return true,
            "m" | "M" => return false,
            _ => {
                eprintln!("{}", "-> Not valid".red());
            }
        }
    }
}

fn read_input_file() -> Result<Vec<String>> {
    let file = fs::OpenOptions::new().read(true).open(INPUT_FILE_PATH)?;

    let reader = BufReader::new(file);
    let mut storage: Vec<String> = Vec::new();

    for line in reader.lines() {
        storage.push(line.unwrap());
    }

    Ok(storage)
}

// seperates the parameter from the variations
fn seperat_strings(storage: Vec<String>) -> Vec<Parameter> {
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

pub fn create_storage_from_file() -> Result<Vec<Parameter>> {
    // process/transfrom input file
    let storage = read_input_file()?;
    let seperate_storage = seperat_strings(storage);

    Ok(seperate_storage)
}

pub fn create_container(
    parameters: &Vec<Parameter>,
) -> BTreeMap<&String, &Vec<String>> {
    // sorted by parameter name
    let mut container: BTreeMap<_, _> = BTreeMap::new();

    for parameter in parameters {
        container.insert(&parameter.name, &parameter.variations);
    }

    container
}

fn get_terminal_width() -> u16 {
    let size = terminal_size();
    let mut screen_width: u16 = 0;

    if let Some((Width(w), Height(_h))) = size {
        screen_width = w;
    } else {
        eprintln!("{}", "Unable to get terminal size".red())
    }

    screen_width
}

pub fn create_table(container: BTreeMap<&String, &Vec<String>>) -> Table {
    if container.is_empty() {
        panic!("No arguments given");
    }

    let datetime = Local::now().to_string();
    let mut idx: i32 = 0;
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(Row::new(vec![
        Cell::new("MORPHBOX").style_spec("FrBdH3bc")
    ]));
    table.add_row(Row::new(vec![Cell::new(&datetime).style_spec("FcH3ic")]));
    table
        .add_row(row![FdBwl->"INDEX", FdBwc->"PARAMETER", FdBwc->"VARIATIONS"]);

    // if table width gets to big, it doesn`t fit into the screen
    // split too long rows into multiple ones
    // with the same parameter name and index
    // TODO split into more than 2 rows if row width is way too big
    let width = get_terminal_width() / 2;

    // TODO works, but this is a mess -> FIXME
    for (key, values) in &container {
        // dont`t start at zero to have extra space
        let mut len: u16 = 20;
        len += key.len() as u16;

        let mut too_long: bool = false;
        let mut temp_str: String = String::new();
        let mut temp_vec: Vec<String> = Vec::new();

        for value in values.into_iter() {
            len += value.len() as u16;
        }

        if len < width {
            for value in values.into_iter() {
                if value == values.into_iter().last().unwrap() {
                    temp_str += value;
                } else {
                    temp_str = temp_str + value + &" | ".to_string();
                }
            }
        } else {
            too_long = true;

            for value in values.into_iter() {
                temp_vec.push(value.to_string());
            }
        }

        if too_long {
            // split value into more rows
            // under same parameter and index
            let mut temp_str1: String = String::new();
            let mut temp_str2: String = String::new();

            let middle = temp_vec.len() / 2;

            for v1 in &temp_vec[0..middle] {
                if v1 == temp_vec[0..middle].last().unwrap() {
                    temp_str1 = temp_str1 + v1;
                } else {
                    temp_str1 = temp_str1 + v1 + &" | ".to_string();
                }
            }

            table.add_row(row![Fy->idx, b->key, c->temp_str1]);

            for v2 in &temp_vec[middle..] {
                if v2 == temp_vec[middle..].last().unwrap() {
                    temp_str2 += v2;
                } else {
                    temp_str2 = temp_str2 + v2 + &" | ".to_string();
                }
            }

            table.add_row(row![Fy->idx, b->key, c->temp_str2]);
        } else {
            table.add_row(row![Fy->idx, b->key, c->temp_str]);
        }

        idx += 1;
    }

    table
}

fn progress_bar(end: u64) -> ProgressBar {
    let pb = ProgressBar::new(end);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{wide_bar:.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );

    pb
}

// create all possible combinations
// output can be ridiculously huge => LIMIT IT
// TODO let user filter out unrealistic combinations to reduce the output -> How?
pub fn combine(lst: &Vec<Parameter>) -> Vec<String> {
    println!("{}", "\n::: Calculating combinations ...".green().bold());
    println!(
        "{}",
        "[This may take a while and the program may seem unresponsive]"
            .red()
            .dimmed()
    );

    let mut all_variations: Vec<Vec<String>> = Vec::new();

    for parameter in lst {
        let var = &parameter.variations;
        all_variations.push(var.to_vec());
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

pub fn get_random_comb() -> bool {
    loop {
        println!("\nGenerate a random combination for further analysis?\n");
        println!("      [ Y ]       => Yes");
        println!("      [ N ]       => No");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {
                eprintln!("{}", "-> Not valid".red());
            }
        }
    }
}

pub fn generate_random_comb(lst: &Vec<String>) -> (u64, String) {
    let len = lst.len();

    if lst.is_empty() || len == 1 {
        let warn: &str = "There is no data left to process.";
        eprintln!("{}", warn.red());
        return (0, "NoData".to_string())
    }

    let r = rand::thread_rng().gen_range(1..len);
    let rand_item = &lst[r];

    (r as u64, rand_item.to_string())
}

pub fn pretty_print_random_comb(param: &Vec<Parameter>, comb: &String) {
    let comb_storage: Vec<&str> = comb.split(",").collect();

    let mut param_storage: Vec<&str> = Vec::new();
    for p in param {
        param_storage.push(&p.name);
    }

    let mut k = 0;
    let mut map: Vec<(&str,&str)> = Vec::new();
    for pair in comb_storage.into_iter().map(|comb| {k += 1; (param_storage[k - 1], comb)}) {
        map.push(pair);
    }
    map.sort();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table
        .add_row(row![FdBwl->"INDEX", FdBwc->"PARAMETER", FdBwc->"VARIATIONS"]);

    let mut i: u8 = 0;
    for (k, v) in map {
        let idx: &str = &i.to_string();
        table.add_row(Row::new(vec![
                Cell::new(idx).style_spec("Fy"),
                Cell::new(&k).style_spec("cb"),
                Cell::new(&v).style_spec("cb")]));
        i += 1;
    }
    
    table.printstd(); 
}

// TODO Finish store / manipulate, ...
pub fn comb_user_options(comb: String, lst: &mut Vec<String>, idx: u64) {
    loop {
        println!("Options:\n");
        println!("      [ R ]       => Remove");
        println!("      [ S ]       => Store");
        println!("      [ M ]       => Manipulate");
        println!("      [ C ]       => Continue");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "r" | "R" => {
                lst.remove(idx as usize);
                println!("Combination index {} successfully removed", idx.to_string().green().bold());
                break;
            }
            "s" | "S" => todo!(),
            "m" | "M" => todo!(),
            "c" | "C" => break,
            _ => {
                eprintln!("{}", "-> Not valid".red());
            }
        }
    }
}

pub fn write_table_to_file(path: &str, table: &Table) -> io::Result<()> {
    println!("{}", "\n::: Creating table ...".green().bold());

    let datetime = Local::now().to_string();
    let new_path = "./output/".to_string() + &datetime + "_" + path;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_path)?;

    writeln!(file, "{}", table)?;

    Ok(())
}

// TODO can take a moment
// -> limit input parameters and variations
// async?
pub fn write_combinations_to_file(
    path: &str,
    lst: &Vec<String>,
) -> io::Result<()> {
    let datetime = Local::now().to_string();
    let new_path = "./output/".to_string() + &datetime + "_" + path;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(new_path)?;

    println!("{}", "\n::: Generating csv file ...".green().bold());
    println!(
        "{}",
        "[This may take a while and the program may seem unresponsive]"
            .red()
            .dimmed()
    );

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
        println!("\nAre you done?");
        println!("      [ Q ]     => Quit");
        println!("      [ N ]     => No, make changes");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "q" | "Q" => return true,
            "n" | "N" => return false,
            _ => {
                eprintln!("{}", "-> Not valid".red());
            }
        }
    }
}

fn clear_screen() {
    println!("\x1Bc");
}

pub fn title() {
    clear_screen();

    let title: String =
        "              /\\/\\0RPH|30X".blue().bold().to_string();
    let mail: String = "        [leann.phydon@gmail.com]".to_string();

    println!("{title}");
    println!("{}\n", mail.dimmed());
}

pub fn warning() {
    let warn_txt: String = "
WARNING! A big number of input variables 
(parameters and their variations) can 
great a huge number of possible combinations 
to calculate and may produce a huge output 
file."
        .to_string();
    let example: String = "Example: 
    10 parameters and 10 variations each 
    generate 10.000.000.000 combinations."
        .to_string();
    let last_warn_txt: String = "You have been warned!".to_string();

    println!("{}", warn_txt.red());
    println!("{}", example.dimmed());
    println!("{}\n\n", last_warn_txt.red().bold());
}

pub fn welcome_and_stop() -> bool {
    title();
    warning();

    println!("      [   Q   ]     => Quit");
    println!("      [ ENTER ]     => Continue");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim() {
        "q" | "Q" => return true,
        _ => {
            return false;
        }
    }
}
