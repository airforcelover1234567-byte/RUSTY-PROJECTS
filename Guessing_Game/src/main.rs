use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number");
    println!("Please input your guess:");

    let secret_number=rand::rng().random_range(1..=100);
    let mut guess=String::new();

    println!("The secret number is {secret_number}");
    io::stdin().read_line(&mut guess).expect("Failed");

    match guess.cmp(&secret_number){
        Ordering::Greater=> println!("Too big"),
        Ordering::Less=> println!("Too small"),
        Ordering::Equal=> println!("You won"),
    }

    println!("You guessed {guess}");
}
