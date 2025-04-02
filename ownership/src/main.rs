fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn gives_ownership() -> String {
    String::from("Hello")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn print_string(a_string: &String) {
    println!("{}", a_string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_2_string(s: &mut String) {
    s.push_str("_$");
}

fn not_dangle() -> String {
    let s = String::from("example2");
    s
}

fn main() {
    let mut s1 = gives_ownership();
    let s_literal = ", world!";
    print_type(&s1);

    let x = 2;
    makes_copy(x);

    let _s2 = String::from("ahoy");
    let s3 = takes_and_gives_back(String::from("zzzzz"));
    let _s4_len = calculate_length(&String::from("Catan"));

    s1.push_str(s_literal);
    append_2_string(&mut s1);
    print_string(&s1);
    print_string(&s1);
    print_string(&s3);
    println!("{}", _s4_len);

    let mut s = String::from("example");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");
    drop(s1);

    let something = not_dangle();
    drop(something);
}
