use std::io::stdin;

fn main() {
    let array = [0; 10];
    let mut input_str = String::new();

    for element in array {
        println!("{}, ", element);
    }
    println!("Input an index: ");
    match stdin().read_line(&mut input_str) {
        Ok(str) => str,
        Err(_) => 0,
    };

    let input: usize = match input_str.trim().parse() {
        Ok(element) => element,
        Err(_) => 0,
    };

    println!("{}, ", array[input]);
    println!("Hello, world!");
}
