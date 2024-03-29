// enum Option<T> {
//     None, 
//     Some(T),
// }

// let some_number = Some(5); // Option<i32>
// let some_char = Some('e'); // Option<char>

// let absent_number: Option<i32> = None;

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }


// enum Coin {
//     Penny, 
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i+1),
//     }
// }

// let five = Some(5);
// let six = plus_one(five);
// let none = plus_one(None);

// let config_max = Some(3u8);
// if let Some(max) = config_max {
//     println!("The maximum is configured to be {}", max);
// }

// let mut count = 0;
// match coin {
//     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//     _ => count +=1,
// }

// let mut count = 0;
// if let Coin::Quarter(state) = coin {
//     println!("State quarter from {:?}!", state);
// } else {
//     count += 1;
// }

fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}

fn main() {
    println!("Hello world");
}