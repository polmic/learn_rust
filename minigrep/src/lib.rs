use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filepath: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Self { query, filepath })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("searching {} in {}", config.query, config.filepath);
    let contents = fs::read_to_string(config.filepath).expect("could not read file");

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
