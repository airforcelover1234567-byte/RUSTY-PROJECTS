fn main() {
    //Scalar data types-These are data types that represent a singular value

    let x:u64=12; //This is a integer type
    let y=3.00; //This is a float type
    let sum=2+3;
    println!("{sum}");

    let t=true;
    let f:bool=false;//Boolean data type

    let c="z"; //THis is a string
    let z:char='Z';//THis is char with adding the type notation

    //Compound Types-Types that represent more than 1 value
    let tup: (u32,f64)=(2,3.000); //THis is a tuple
    // let (x,y) =tup; -THis is called destructing
    let x=tup.0;
    let y=tup.1;
    println!("{x},{y}");

    let list: [i32; 5]=[1,2,3,4,5]; //This is an array
    let a=[3;5]; //This makes all the elements of an array as 3

    let first=list[0];
    let second=list[1];
    println!("{first} {second}");

}