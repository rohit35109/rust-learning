use std::io;
use rand::prelude::*;

// This is the main function that gets called first
fn main() {
    println!("Welcome to the Guessing Game");

    let secret_numbner = rand::rng().random_range(1..=100);

    println!("The secret number is {secret_numbner}");

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}