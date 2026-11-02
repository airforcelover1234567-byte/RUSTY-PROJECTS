use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("The values of the array are [1,2,3,4,5,6,7,8]");

    let mut input = String::new();
    println!("Enter the index of the element of the array you want");
    io::stdin().read_line(&mut input).expect("Failed");
    let input: usize = input.trim().parse().expect("Failed");

    let number = array[input];
    println!("The element you took out from the array is {number}");
}
