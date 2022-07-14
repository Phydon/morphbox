use colored::*;
use prettytable::Table;

use std::{collections::BTreeMap, process};

use morphbox::*;

const FILEPATH_TABLE: &str = "mymorphbox.txt";
const FILEPATH_COMBINATIONS: &str = "mycombinations.csv";
const FILEPATH_STORAGE: &str = "mystorage.csv";

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
        if input_from_file() {
            match create_storage_from_file() {
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
        let mut comb_lst = combine(&parameters);
        let mut future_comb_storage: Vec<String> = Vec::new();

        while get_random_comb() {
            let (idx, rand_output) = generate_random_comb(&comb_lst);

            if idx == 0 && rand_output == "NoData" {
                break;
            } else {
                println!("Combination Index:: {}", idx.to_string().blue().bold());
                pretty_print_random_comb(&parameters, &rand_output);
                // println!("\n  =>  {}\n", rand_output.bold());

                // TODO more options for user:
                comb_user_options(rand_output, &mut comb_lst, idx, &mut future_comb_storage);
            }
        }

        // TODO choose proper file format to display the table
        if are_u_done() {
            write_table_to_file(FILEPATH_TABLE, &table)
                .expect("Failed to write table to file");
            write_combinations_to_file(FILEPATH_COMBINATIONS, &comb_lst)
                .expect("Failed to write combinations to file");
            write_future_comb_storage_to_file(FILEPATH_STORAGE, &future_comb_storage)
                .expect("Failed to write storage to file");
            break;
        }
    }

    process::exit(0);
}
