use std::io;

fn main() {
    /*
    let _guess: u32 = "42".parse().expect("Not a number!");
    let _x = 5;
    println!("The value of x is: {_x}");
    let _x = 6;
    {
        let _x = 7;
        println!("The value of x is: {_x}");
        println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    }
    */
    /*
    let _sum = 5 + 15;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _truncated = -5 / 3; // results in -1

    let _remainder = -1 % 3;

    println!("-1 % 3 = {_remainder}");
    */
    the_function();
}

fn the_function() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
