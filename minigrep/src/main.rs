use std::{env, process};
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
