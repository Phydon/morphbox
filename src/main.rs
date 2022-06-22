use morphbox::*;

fn main() {
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

    for param in parameters {
        println!("name: {:?}", param.name);
        println!("variations: {:?}", param.variations);
    }
}
