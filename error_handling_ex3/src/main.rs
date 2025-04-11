use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::net::IpAddr;

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let result = read_username_from_file_shorter();
    match result {
        Ok(username) => println!("the username is {}", username),
        Err(error) => println!("the error is {}", error),
    }
    let last_char = last_char_of_first_line("Hello world").unwrap();
    println!("last_char: {}", last_char);
    // let last_char = last_char_of_first_line("").unwrap();
    // println!("last_char: {}", last_char.to_string());
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{}", home);
}
