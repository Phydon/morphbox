#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub variations: Vec<String>,
}

impl Parameter {
    pub fn new(word: String, var: Vec<String>) -> Result<Parameter, &'static str> {
        if var.len() <= 0 {
            return Err("not enough arguments")
        }
        let name = word;
        let variations = var;

        Ok(Parameter {name, variations})
    }
}
