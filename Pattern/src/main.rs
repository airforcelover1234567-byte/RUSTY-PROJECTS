use std::io;

fn main() {
    let mut input=String::new();
    println!("Enter the number you want to pattern to continue: ");
    io::stdin().read_line(&mut input).expect("Failed");
    let input:u64=input.trim().parse().expect("Failed");

    for i in 1..=input{
        for j in 1..=i{
            print!("{j}");
        }
        println!("\n");
    }
}
