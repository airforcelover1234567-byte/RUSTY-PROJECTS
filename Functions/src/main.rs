fn another_function(x: u64) {
    println!("This is an example of a function");
    println!("The value of x is {x}");
}
fn measurements(x: u64, y: char) {
    println!("The length of the table is {x}{y}")
}
fn five() -> u64 {
    5
}
//Statement-Simply, a function that does not return a value, like void functions
//Expression-Simply, a function that returns a value

fn main() {
    another_function(5);
    measurements(5, 'm');

    let y = {
        let x = 3;
        x + 1
    };
    println!("THe value of y is {y}");
    let x = five();
    println!("The value of x is {x}");
}
