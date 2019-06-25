extern crate rand;

use http::{Request, Response};

use std::thread;
use std::time::SystemTime;
fn main() {
    process_http_requests();
    // println!("Please guess a number between 1 and 100");
    // let secret_number = rand::thread_rng().gen_range(1, 101);

    // let mut guess = String::new();
    // io::stdin()
    //     .read_line(&mut guess)
    //     .ok()
    //     .expect("Failed To Read Line");
    // println!(
    //     "You Guessed {} and secret number was {}",
    //     guess, secret_number
    // );
}

fn process_http_requests() {
    let mut vect_of_names = Vec::new();
    vect_of_names.push("Vikas");
    vect_of_names.push("Diksha");
    vect_of_names.push("Viyona");


    for i in vect_of_names {
        thread::spawn(move || {
            print!("This is the Value of i {} \n at {:?}", i, SystemTime::now());
        });
    }
}