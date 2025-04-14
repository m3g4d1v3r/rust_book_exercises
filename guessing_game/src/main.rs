use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
    pub fn value(self: &Guess) -> i32 {
        self.value
    }
    pub fn cmp(self: &Guess, secret_number: &i32) -> Ordering {
        if self.value < *secret_number {
            Ordering::Less
        } else if self.value > *secret_number {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number:");

        let mut guess_str = String::new();

        match stdin().read_line(&mut guess_str) {
            Ok(str) => str,
            Err(_) => continue,
        };

        let guess: Guess = match guess_str.trim().parse() {
            Ok(element) => Guess::new(element),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess.value);

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
