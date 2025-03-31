use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number:");

        let mut guess_str = String::new();

        match stdin().read_line(&mut guess_str) {
            Ok(str) => str,
            Err(_) => continue,
        };

        let guess: u32 = match guess_str.trim().parse() {
            Ok(element) => element,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Found it!");
                break;
            }
        }
    }
}
