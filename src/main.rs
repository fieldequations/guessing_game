use std::io;
use rand::Rng;

fn main() {
    println!("Guess your number");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {}", guess);
    println!("this may be right, let me check")
}
