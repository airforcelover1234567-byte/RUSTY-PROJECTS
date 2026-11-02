use std::io;
fn sum(num1:u64, num2:u64)->u64{
    num1+num2
}
fn main() {
    let mut num1=String::new();
    let mut num2=String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1).expect("Failed");
    let num1:u64=num1.trim().parse().expect("Failed");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut num2).expect("Failed");
    let num2:u64=num2.trim().parse().expect("Failed");

    let sum=sum(num1,num2);
    println!("The sum of the 2 numbers are {sum}");
}