use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer_pt = Point { x: 5, y: 10 };
    let float_pt = Point { x: 1.0, y: 4.0 };
    println!("Integer Point: x_{} y_{}", integer_pt.x(), integer_pt.y());

    println!("Float Point: x_{} y_{}", float_pt.x, float_pt.y);
    println!("Distance from origin: {}", float_pt.distance_from_origin());

    let str_a = String::from("abdc");
    let str_b = "xyz";
    println!("Longest str: {}", longest(str_a.as_str(), str_b));
    println!(
        "2 Longest str: {}",
        longest_with_an_announcement(str_a.as_str(), str_b, "hello annotation")
    );
}
