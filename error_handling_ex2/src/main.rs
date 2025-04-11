use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    // File is a file handle
    // T is File::open
    // E is std::io::Error
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _other_error => panic!("Problem opening the file: {error:?}"),
        },
    };

    println!("It worked!");
}
