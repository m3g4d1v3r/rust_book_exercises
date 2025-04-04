fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let _data_string = data.to_string();
    let _data_string = "initial_contents".to_string();
    let data_string = String::from("initial_contents");

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שלום");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let mut hello = String::from("Hola");
    hello.push_str(" que tal?");

    println!("{}", data_string);
    println!("{}", hello);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s equals: {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let placeholder = s1.clone();
    let s3 = s1 + &s2;
    println!("s1 is {placeholder}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("hello");
    for c in s1.chars() {
        println!("{c}");
    }
    println!();
    for c in "Зд".chars() {
        println!("{c}");
    }
    println!();
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
