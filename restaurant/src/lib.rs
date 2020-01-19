mod old_front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        mod back_of_house {
            fn cook_order() {}

            fn fix_incorrect_order() {
                cook_order();
                // start relative path with 'super' which points to the parent module
                super::serve_order();
            }
        }
    }
}

mod back_of_house {
    // make some fields of a struct public and keep the rest private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // 'pub' on an enum makes all variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// bring module into the current scope (also possible with relative path)
use crate::old_front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::old_front_of_house::hosting::add_to_waitlist();

    // relative path
    old_front_of_house::hosting::add_to_waitlist();

    // possible after 'use'
    hosting::add_to_waitlist();

    // order a breakfast in the summer
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // next line won't compile; we are not allowed to see or modify
    // the seasonal_fruit field
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// bring module from different file into the scope
mod front_of_house;

pub use crate::front_of_house::hosting as new_hosting;

pub fn new_eat_at_restaurant() {
    new_hosting::add_to_waitlist();
    new_hosting::add_to_waitlist();
    new_hosting::add_to_waitlist();
}
