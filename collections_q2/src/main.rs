fn pig_latin(input: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    let mut chars = input.chars();
    let first_char = chars.next().expect("There's no first char on input");
    for vowel in vowels {
        if first_char == vowel {
            return format!("{}-hay", input.to_string());
        }
    }
    format!("{}-{}ay", chars.collect::<String>(), first_char)
}

fn main() {
    assert!(pig_latin("first") == "irst-fay");
    assert!(pig_latin("apple") == "apple-hay");
}
