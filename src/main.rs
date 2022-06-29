use prettytable::Table;

use std::{
    collections::BTreeMap,
    process,
};

use morphbox::*;

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
                println!("Unable to process parameters and variations from file\n");
            }
        }

        // if no file was given, manually enter parameters and variations
        if no_file {
            println!("Enter parameters and variations manually:\n");
            parameters = cycle_inputs();
        }

        let container: BTreeMap<&String,&Vec<String>> = create_container(&parameters);

        let table: Table = create_table(container);

        table.printstd();
        let lst = combine(parameters);

        if are_u_done() {
            write_to_file(&table, &lst).expect("Failed to write to file");
            break;
        }
    }

    process::exit(0);
}
