fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..idx];
        }
    }
    &s[0..s.len()]
}

fn second_word(s: &str) -> &str {
    let slice = first_word(&s);
    let first_idx = slice.len() + 1;
    let bytes = slice.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[first_idx..idx];
        }
    }
    &s[first_idx..s.len()]
}

fn main() {
    let _chr: char = 'a';
    let the_string = String::from("Hello World ");
    let mut word = first_word(&the_string);
    println!("{}", word);
    word = second_word(&the_string);
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
