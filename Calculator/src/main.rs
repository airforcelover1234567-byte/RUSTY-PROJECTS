use std::io;

fn menu() {
    println!("1.Add  2.Subtract  3.Multiply  4.Divide  5.Exit")
}
fn sum(num1: f64, num2: f64) -> f64 {
    num1 + num2
}
fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}
fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}
fn divide(num1: f64, num2: f64) -> f64 {
    num1 / num2
}
fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut choice = String::new();

    menu();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1).expect("Failed");
    let num1: f64 = num1.trim().parse().expect("Failed");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut num2).expect("Failed");
    let num2: f64 = num2.trim().parse().expect("Failed");

    println!("Enter the operation you want to perform: ");
    io::stdin().read_line(&mut choice).expect("Failed");
    let choice: u64 = choice.trim().parse().expect("Failed");

    match choice {
        1 => {
            let x = sum(num1, num2);
            println!("The sum of the 2 numbers are {x}");
        }
        2 => {
            let x = subtract(num1, num2);
            println!("The difference of the 2 number are {x}");
        }
        3 => {
            let x = multiply(num1, num2);
            println!("The product of the 2 numbers are {x}");
        }
        4 => {
            let x = divide(num1, num2);
            println!("The quotient of the 2 numbers are {x}");
        }
        _ => {
            println!("Invalid Input. Enter a proper command");
        }
    }
}
