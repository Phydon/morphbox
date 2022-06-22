use morphbox::*;

fn main() {
    let parameter_name = input();
    let args = input_variations();

    let param = Parameter::new(parameter_name, args);

    println!("param: {:?}", param);
    println!("name: {:?}", param.name);
    println!("variations: {:?}", param.variations);
}
