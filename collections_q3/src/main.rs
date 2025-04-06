use clearscreen::clear;
use regex::Regex;
use std::collections::BTreeMap;
use std::io::stdin;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn parse_key_value(user_input: String) -> (String, String) {
    let re = Regex::new(r"Add (\w+) to (\w+)").unwrap();
    let result = re.captures(&user_input);
    print_type_of(&result);
    match result {
        Some(data) => {
            let name: String = data
                .get(1)
                .expect("Something wrong went into parsing the 1st parameter")
                .as_str()
                .to_string();
            let department: String = data
                .get(2)
                .expect("Something wrong went into parsing the 2nd parameter")
                .as_str()
                .to_string();
            println!("key: {name}, value: {department}");
            return (name, department);
        }
        None => {
            println!("The input string does not match the regex!");
        }
    }
    (String::from(""), String::from(""))
}

fn print_interface() {
    clear().expect("Failed to clear the screen!");
    println!("[a]: add entry");
    println!("[l]: list everything");
    println!("[q]: quit");
}

fn read_input() -> String {
    let mut user_input = String::new();
    match stdin().read_line(&mut user_input) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("An error in the I/O has occurred!");
            0
        }
    };
    user_input = user_input.trim().to_string();
    user_input
}

fn press_enter_to_continue() {
    println!("Press enter to continue");
    read_input();
}

fn main() {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut rev_map: BTreeMap<String, Vec<String>> = BTreeMap::new();

    println!("Welcome to CLI");
    loop {
        print_interface();
        let user_input = read_input();
        if user_input == "a" {
            println!("Add entry in the format: \"Add <name> to <department>\"");
            let user_input = read_input();
            let tup = parse_key_value(user_input);
            press_enter_to_continue();
            if tup.0 == "" && tup.1 == "" {
                continue;
            }
            map.insert(tup.0.clone(), tup.1.clone());
            match rev_map.get_mut(&tup.1) {
                Some(vector) => {
                    vector.push(tup.0.clone());
                    vector.sort();
                }
                None => {
                    rev_map.insert(tup.1.clone(), vec![tup.0.clone()]);
                }
            };
        } else if user_input == "l" {
            for (department, names) in rev_map.iter() {
                println!("Department: {department}");
                for name in names {
                    println!("{name}");
                }
                println!();
            }
            press_enter_to_continue();
        } else if user_input == "q" {
            break;
        }
    }
}
