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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
