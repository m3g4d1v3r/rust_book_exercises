const LYRICS: [&str; 14] = [
    "On the {verse} day of Christmas",
    "My true love sent to me",
    "A partridge in a pear tree",
    "Two turtle-doves",
    "Three French hens",
    "Four calling birds",
    "Five golden rings (five golden rings)",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "I sent 11 pipers piping",
    "12 drummers drumming",
];

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth",
    "11th", "12th",
];

fn print_xmas_carol() {
    for verse in 0..12 {
        println!("{}", LYRICS[0].replace("{verse}", DAYS[verse]));
        println!("{}", LYRICS[1]);
        for inner_line in 2..(verse + 3) {
            println!("{}", LYRICS[inner_line]);
        }
        println!();
    }
}

fn main() {
    print_xmas_carol();
}
