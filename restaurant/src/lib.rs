fn deliver_order() {}

mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
        super::back_of_house::fix_incorrect_order();

        let mut meal = super::front_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        // meal.seasonal_fruit = String::from("grapes");
        println!("I'd like {} toast please", meal.toast);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        customer::eat_at_restaurant();
    }
}
