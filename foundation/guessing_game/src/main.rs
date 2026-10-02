use std::io;

// This is the main function that gets called first
fn main() {
    println!("Welcome to the Guessing Game");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}