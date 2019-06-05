use std::io;
fn main() {
    println!("Please type something and then hit enter");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Failed To Read Line");
    println!("You Guessed : {}", guess);
}

