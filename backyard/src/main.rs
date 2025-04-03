use crate::garden::vegetables::Aspargus;

pub mod garden;

fn main() {
    let plan = Aspargus {};
    println!("I'm growing {plan:?}!");
}
