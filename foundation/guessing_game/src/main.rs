use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

// This is the main function that gets called first
fn main() {
    println!("Welcome to the Guessing Game");

    let secret_number = rand::rng().random_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too Large."),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}