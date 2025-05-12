use std::io;
use rand::Rng;

fn generate_secret() -> i32 {
    rand::rng().random_range(1..=100)
}

fn main() {
    let secret = generate_secret();

    println!("{secret}");

    println!("Guess the number:");
    println!("Input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess)
}