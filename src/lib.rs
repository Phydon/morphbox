use std::error::Error;

pub struct Config {
    pub file: String,
}

impl Config {
    pub fn new(name: String) -> Result<Config, Box<dyn Error>> {
        let filename = name;

        Ok(Config{file: filename})
    }
}

pub fn run(config: Config) {
    println!("{}", config.file);
}
