fn main() {
    println!("Hello, world!");
    another_function(10);
    let x = five();
    println!("x: {x}");

    if x < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let _number = if condition { 5 } else { 6 };
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("[{index}]: {}", a[index]);
        index += 1;
    }
    for element in a {
        println!("[]: {}", element);
    }

    println!("Countdown");
    let nbs = vec![5, 4, 3, 2, 1];
    for (idx, element) in nbs.iter().enumerate() {
        println!("[{idx}]: {element}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}
