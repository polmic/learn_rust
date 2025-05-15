use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn generate_secret() -> u8 {
    rand::rng().random_range(1..=100)
}

fn compare_secret_to_guess(secret: u8, guess: u8) -> Ordering {
    match guess.cmp(&secret) {
        Ordering::Less => {
            println!("Too small!");
            Ordering::Less
        },
        Ordering::Greater => {
            println!("Too big!");
            Ordering::Greater
        },
        Ordering::Equal => {
            println!("You win!");
            Ordering::Equal
        },
    }
}

fn main() {
    let secret = generate_secret();

    println!("Guess the number:");
    println!("Input your guess");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u8 = guess.trim().parse().expect("Not a valid u8 number");
        
        if compare_secret_to_guess(secret, guess) == Ordering::Equal {
            break;
        }
    }
}