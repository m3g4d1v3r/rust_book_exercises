enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: IpAddr) {
    match ip {
        IpAddr::V4(tp0, tp1, tp2, tp3) => {
            println!("Routing to IPv4 address {tp0}.{tp1}.{tp2}.{tp3}")
        }
        IpAddr::V6(address) => println!("Routing to IPv6 address {address}"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {}
            Message::Move { x: _, y: _ } => {}
            Message::Write(message) => println!("{}", message),
            Message::ChangeColor(_, _, _) => {}
        }
    }
}

fn print_option<T>(value: &Option<T>) {
    match value {
        Some(_value) => println!("There's a value"),
        None => println!("None"),
    };
}

fn print_char<Char: ToString>(value: &Option<Char>) {
    match value {
        Some(value) => println!("There's a value {}", value.to_string()),
        None => println!("None"),
    };
}

fn print_unsigned32<U32: ToString>(value: &Option<U32>) {
    match value {
        Some(value) => println!("There's a value {}", value.to_string()),
        None => println!("None"),
    };
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // None => None,
        // _other => None,
        _ => None,
    }
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1990) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    route(home);
    route(loopback);
    println!("Hello, world!");

    let msg = Message::Write(String::from("Message to be sent to point B"));
    msg.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    print_unsigned32(&some_number);
    print_char(&some_char);
    print_option(&absent_number);

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let the_coin = Coin::Penny;
    let mut count = 0;

    if let Coin::Quarter(ref state) = the_coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    let result = describe_state_quarter(&the_coin);
    if let Some(format_str) = result {
        println!("{}", format_str);
    }

    println!("Count: {count}");
}
