use prettytable::Table;

use std::{
    collections::BTreeMap,
    process,
};

use morphbox::*;

fn main() {
    // TODO get this into the loop
    //
    // TODO if no input file exists, continue with manual looping input
    let storage: Vec<Parameter> = create_storage().expect("Error processing input from file");

    // TODO double use of variable container
    let container: BTreeMap<&String,&Vec<String>> = create_container(&storage);


    loop {
        let parameters: Vec<Parameter> = cycle_inputs();
        // for param in &parameters {
        //     println!("name: {}", param.name);
        //     println!("variations: {:?}", param.variations);
        // }

        let container: BTreeMap<&String,&Vec<String>> = create_container(&parameters);
        // for (key, value) in container {
        //     println!("key: {key}");
        //     println!("value: {:?}", value);
        // }

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
