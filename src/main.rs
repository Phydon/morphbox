use prettytable::Table;

use std::{
    collections::BTreeMap,
    process,
};

use morphbox::*;

fn main() {
    loop {
        let parameters: Vec<Parameter> = cycle_inputs();
        // for param in &parameters {
        //     println!("name: {}", param.name);
        //     println!("variations: {:?}", param.variations);
        // }

        let container: BTreeMap<String,Vec<String>> = create_container(parameters);
        // for (key, value) in container {
        //     println!("key: {key}");
        //     println!("value: {:?}", value);
        // }

        let table: Table = create_table(container);

        let done = are_u_done(&table);
        if done {
            write_to_file(&table).expect("Failed to write to file");
            break;
        }
    }

    process::exit(0);
}
