use std::io; // importing io module from standard library

fn first_function(){
    println!("first function");

}
fn add_numbers(x:i32 , y:i32){
    println!("sum is: {}",x + y)
}

fn main() {
    let x: u32 = 4;
    println!("x is: {}", x);
    // x = 5 can not reassign x without redeclaring it , have to define it as mut
    add_numbers(5,3);
    {
        let x: u32 = 2; // scope of x, x is only = 2 when it is inside of these curly braces
        println!("x is: {}", x);
    }
    let mut y: u32 = 5;
    y = 6;
    let x = "hello"; // redefining it so it can have a differnt type, can not change the type if I
                     // did x = "hello"
    println!("y is: {}", y);
    println!("x is: {}", x);
    const SECONDS_IN_MINUTE: u32 = 60;
    //can not redefine constants const SECONDS_IN_MINUTE = 100, would not work
    println!("Seconds in a minute: {}", SECONDS_IN_MINUTE);
    let tup: (i32, bool, char) = (1, true, 'c'); // immutable, specific type
    let mut tup2: (i8, bool, char) = (1, true, 'c');
    println!("first element in tuple2 is: {}", tup2.0);
    tup2.0 = 11;
    println!("first element got mutated is now: {}", tup2.0);
    println!("first element in tuple is: {}", tup.0);
    let mut arr = [1, 2, 3, 4, 5]; // have all have to be the same type
    println!("first element in array is: {}", arr[0]);
    arr[0] = 10;
    println!("mutated first element is now: {}", arr[0]);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read the line"); // this will read the line from the terminal
    println!("we have written: {}", input);

    let x:i8 = 13;
    let y: i8 = -3;
    let z = x + y ; 
    println!("z is : {}",z);
    first_function();
    
}
