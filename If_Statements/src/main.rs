fn main() {
    let number = 3;
    if number < 5 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }
    let bolo = true;
    if bolo {
        println!("You are online");
    } else {
        println!("You are offline");
    }
    let num = 2;
    if num % 2 == 0 {
        println!("The number is divisible by 2")
    } else if num % 3 == 0 {
        println!("The number is divisible by 3");
    } else if num % 5 == 0 {
        println!("The number is divisible by 5");
    } else {
        println!("The number is not divisible by the availabe numbers");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is {number}");
}
