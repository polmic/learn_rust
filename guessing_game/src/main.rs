use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn generate_secret() -> u32 {
    rand::rng().random_range(1..=100)
}

fn compare_secret_to_guess(secret: u32, guess: u32) {
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn main() {
    let secret = generate_secret();

    println!("{secret}");

    println!("Guess the number:");
    println!("Input your guess");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    
    let guess: u32 = guess.trim().parse().expect("Not a valid positive number");
    
    compare_secret_to_guess(secret, guess);
}