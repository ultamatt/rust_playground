// Chapter 7
// CRATES!
// Two types of projects, binaries and libraries
//  Binaries have a main.rs
//  Libraries don't!
// -> It's possible to have both a main.rs and a lib.rs, which means your package has two crates!


mod front_of_house {
    //Public modules can be used by folks using this crate
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("You're on the waitlist");
        }

        fn seat_at_table() {}
    }
    //Private modules can't be used outside this library file
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn say_order(&self) {
            println!("I'd like {} toast please", self.toast);
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // With the 'use' above
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    meal.say_order();

    //meal.toast = String::from("Cinnamon");
    //meal.say_order();

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
