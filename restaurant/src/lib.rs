// // mod front_of_house {
// //     pub mod hosting {
// //         pub fn add_to_waitlist() {}

// //         fn seat_at_table() {}
// //     }

// //     mod serving {
// //         fn take_order() {}
// //         fn serve_order() {}
// //         fn take_payment() {}
// //     }
// // }
// // // //crate
// // //  └── front_of_house
// // //  ├── hosting
// // //  │   ├── add_to_waitlist
// // //  │   └── seat_at_table
// // //  └── serving
// // //      ├── take_order
// // //      ├── serve_order
// // //      └── take_payment

// // fn deliver_order() {}

// // mod back_of_house {
// //     fn fix_incorrect_order() {
// //         cook_order();
// //         super::deliver_order();
// //     }
// //     fn cook_order() {}

// //     pub enum Appetizer {
// //         Soup,
// //         Salad,
// //     }

// //     pub struct Breakfast {
// //         pub toast: String,
// //         seasonal_fruit: String,
// //     }

// //     impl Breakfast {
// //         pub fn summer(toast: &str) -> Breakfast {
// //             Breakfast {
// //                 toast: String::from(toast),
// //                 seasonal_fruit: String::from("peaches"),
// //             }
// //         }
// //     }
// // }

// // pub fn eat_at_restaurant() {
// //     // Order a breakfast in the summer with Rye toast
// //     let mut meal = back_of_house::Breakfast::summer("Rye");
// //     // Change our mind about what bread we'd like
// //     meal.toast = String::from("Wheat");
// //     println!("I'd like {} toast please", meal.toast);

// //     let order1 = back_of_house::Appetizer::Soup;
// //     let order2 = back_of_house::Appetizer::Salad;
// // }

// // mod front_of_house {
// //     pub mod hosting {
// //         pub fn add_to_waitlist() {}
// //     }
// // }

// // use crate::front_of_house::hosting;

// // mod customer {
// //     use crate::front_of_house::hosting;

// //     pub fn eat_at_restaurant() {
// //         hosting::add_to_waitlist();
// //     }
// // }

// // use std::fmt::Result;
// // use std::io::Result as IoResult;

// // fn function1() -> Result {
// //     //-- snip --
// // }

// // fn function2() -> IoResult<()> {}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub use crate::front_of_house::hosting;

// fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// // --snip--
// use std::cmp::Ordering;
// use std::io;
// // --snip--

// // --snip--
// use std::{cmp::Ordering, io};
// // --snip--


// // --snip--
// use std::io;
// use std::io::Write;
// // --snip--

// // --snip--
// use std::io::{self, Write};

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}