#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn print_user(input: &User) {
    println!("{}", input.active);
    println!("{}", input.username);
    println!("{}", input.email);
    println!("{}", input.sign_in_count);
    println!();
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    /*
    let user1 = User {
        sign_in_count: 1,
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
    };
    */
    let user1: User = build_user(String::from("Jack"), String::from("jack@domain.com"));
    let user2 = User {
        username: String::from("Vaporware"),
        email: String::from("another@example.com"),
        ..user1.clone()
    };
    print_user(&user1);
    print_user(&user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);
}
