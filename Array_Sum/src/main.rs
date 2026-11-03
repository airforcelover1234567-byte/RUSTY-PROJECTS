use std::io;

fn main() {
    let mut target = String::new();
    let array: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let i: u32 = 0;
    // let j:u32=0;

    println!("Enter the target: ");
    io::stdin().read_line(&mut target).expect("Failed");
    let target: i32 = target.trim().parse().expect("Failed");

    if target > 9 {
        println!("Enter a number less than or equal to 9");
    } else {
        for i in 0..9 {
            for j in 0..9{
                if array[i]+array[j]==target{
                    println!("The array elements of {} and {} add up to target",array[i],array[j]);
                }
            } 
        }
    }
}
