use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("Guess the Number");
    let tries=5;
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
        }
    }
}
