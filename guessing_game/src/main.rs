use std::io::{self, Write};
use rand::Rng;

// Program developed using The Rust Programming book as an exercise
fn main() {
    println!("Guess the number!");

    print!("Please input your guess: ");

    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line :(");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("");

    println!("You guessed: {}", guess);

    println!("The secret number is: {}", secret_number);
}
