fn main() {
    let mut x = 5;
    println!("THe value of x is {x}");
    x = 6; //Cannot change value of x since variables are immutable
    println!("The value of x is {x}");
    const Y: u32 = 69; //We cannot use mut here as constants are supposed to be immutable
    println!("{Y}");

    //SHADOWING!

    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is {z}");
    }
    println!("The value of z is {z}");
}
