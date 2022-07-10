use colored::*;
use prettytable::Table;

use std::{collections::BTreeMap, process};

use morphbox::*;

const FILEPATH_TABLE: &str = "mymorphbox.txt";
const FILEPATH_COMBINATIONS: &str = "mycombinations.csv";

// TODO limit the number of parameters <= 8 ???
// and variations <= 8 ???
// both in the input file and the manual input,
// otherwise a huge file will be generated and it takes forever
// to calculate combinations and write to file
fn main() {
    if welcome_and_stop() {
        process::exit(1)
    }

    loop {
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut no_file: bool = true;

        // read parameters and variations from a file
        // TODO ask user for filepath -> than process it
        match create_storage() {
            Ok(storage) => {
                no_file = false;
                parameters = storage;
            }
            _ => {
                println!(
                    "{}",
                    "Unable to process parameters and variations from 
                    file\n"
                        .red()
                );
            }
        }

        // if no file was given, manually enter parameters and variations
        if no_file {
            println!("Enter parameters and variations manually:\n");
            parameters = cycle_inputs();
        }

        let container: BTreeMap<&String, &Vec<String>> =
            create_container(&parameters);

        let table: Table = create_table(container);

        table.printstd();
        let mut lst = combine(parameters);

        while get_random_comb() {
            let (idx, rand_output) = generate_random_comb(&lst);
            // TODO pretty print as table
            println!("\n  =>  {}\n", rand_output.bold());

            // TODO options for user:
            comb_user_options(rand_output, &mut lst, idx);
        }

        // TODO choose proper file format to display the table
        if are_u_done() {
            write_table_to_file(FILEPATH_TABLE, &table)
                .expect("Failed to write table to file");
            write_combinations_to_file(FILEPATH_COMBINATIONS, &lst)
                .expect("Failed to write combinations to file");
            break;
        }
    }

    process::exit(0);
}
