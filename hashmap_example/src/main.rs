use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let team_blue_string = String::from("Blue");
    let team_yellow_string = String::from("Yellow");
    scores.insert(team_blue_string.clone(), 10);
    scores.insert(team_blue_string.clone(), 20);
    scores.insert(team_yellow_string, 50);

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    let blue_score = scores.get("Blue").copied().unwrap_or(0);
    println!("{}, blue_score: {}", team_blue_string, blue_score);
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split(' ') {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
