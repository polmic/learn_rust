use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("arguments must be query and filepath");
        }
        Ok(Self {
            query: args[1].clone(),
            filepath: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("searching {} in {}", config.query, config.filepath);
    let contents = fs::read_to_string(config.filepath).expect("could not read file");
    println!("{}", contents);

    Ok(())
}