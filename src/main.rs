extern crate rand;

use rand::Rng;
use std::io;
fn main() {
    println!("Please guess a number between 1 and 100");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Failed To Read Line");
    println!(
        "You Guessed {} and secret number was {}",
        guess, secret_number
    );
}

