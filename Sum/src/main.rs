use std::io;

fn main() {
    println!("Prorgram to add 2 numbers");

    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter the first number");
    io::stdin().read_line(&mut num1).expect("Failed");
    let num1: u32 = num1.trim().parse().expect("Failed");

    println!("Enter the first number");
    io::stdin().read_line(&mut num2).expect("Failed");
    let num2: u32 = num2.trim().parse().expect("Failed");

    println!("The sum of the number are {}", num1 + num2);
}
