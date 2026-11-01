use std::io;

fn main() {
    println!("==TREASURE==");
    const ID: u32 = 2026;
    println!("Vault Id: {ID}");

    let balance = 100;
    println!("Balance: {balance}");
    let mut depo = String::new();
    let mut with = String::new();

    println!("Enter the number of gold coins you want to deposit: ");
    io::stdin().read_line(&mut depo).expect("Failed");
    let depo: u32 = depo.trim().parse().expect("Failed");
    println!("Deposit: {depo}");

    println!("Enter the number of gold coins you want to Withdraw: ");
    io::stdin().read_line(&mut with).expect("Failed");
    let with: u32 = with.trim().parse().expect("Failed");
    println!("Withdraw: {with}");

    let balance = balance + depo - with;
    println!("FInal Balance: {balance}");
}
