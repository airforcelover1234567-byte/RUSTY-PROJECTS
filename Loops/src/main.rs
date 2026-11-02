fn main() {
    // loop{
    //     println!("Again this stupid loop");
    // }
    let mut number=3;
    while number!=0{
        println!("{number}!");
        number-=1;
    }
    let a=[1,2,3,4,5,6,7];

    for element in a{
        println!("The value of the element is {element}");
    }
    for number in(1..4){
        println!("{number}");
    }
}
