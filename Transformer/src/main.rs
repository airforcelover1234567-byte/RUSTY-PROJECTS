use std::io;

fn main() {
    let mut data = String::new();
    println!("Enter a data that you want:");
    io::stdin().read_line(&mut data).expect("Failed");

    let data: u64 = data.trim().parse().expect("Failed");
    let data = data * 2;

    let data = data as f64;
    let data = data / 2.0;

    let data = data.to_string();
    println!("{data}");
}
