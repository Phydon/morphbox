use prettytable::Table;
use colored::*;

use std::{
    collections::BTreeMap,
    process,
};

use morphbox::*;

const FILEPATH_TABLE: &str = "mymorphbox.txt";
const FILEPATH_COMBINATIONS: &str = "mycombinations.csv";

// TODO limit the number of parameters <= 8 ???
// and variations <= 8 ???
// both in the input file and the manual input,
// otherwise a huge file will be generated and it takes forever 
// to calculate combinations and write to file
fn main() {
    loop {
        let mut parameters: Vec<Parameter> = Vec::new();
        let mut no_file: bool = true;

        // read parameters and variations from a file
        // TODO ask user for filepath -> than process it
        match create_storage() {
            Ok(storage) => {
                no_file = false;
                parameters = storage;
            },
            _ => {
                println!("{}", "Unable to process parameters and variations from 
                    file\n".red());
            }
        }

        // if no file was given, manually enter parameters and variations
        if no_file {
            println!("Enter parameters and variations manually:\n");
            parameters = cycle_inputs();
        }

        let container: BTreeMap<&String,&Vec<String>> = 
            create_container(&parameters);

        let table: Table = create_table(container);

        // TODO fix for large tables
        // width to big 
        //      -> doesn`t fit to screen
        //      -> doesn`t fit into file
        // SOLUTION for variations:: calculate used window size (of the monitor)
        // calculate max number of possible variations in one line
        // if limit for a line is reached -> message to user
        // -> create the same parameter (automate this for the user) and 
        // start again asking for variations
        // same for the input file 
        // -> automatically create a new line with the same parameter
        // => no solution for max parameters (is this needed??)
        table.printstd();
        let lst = combine(parameters);

        while get_random_comb() {
            let rand_output = generate_random_comb(&lst);
            println!("\n  =>  {}\n", rand_output.bold());
            
            // TODO options for user:
            // remove
            // do something else with the randomly generated combination
            // todo!();
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
