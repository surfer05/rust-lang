use std::fs::File;
// // use std::io::ErrorKind;
// fn main() {
//     // let v = vec![1, 2, 3];
//     // v[99];

//     // Method 1 : more verbose

//     // let greeting_file_result = File::open("hello.rs");

//     // let greeting_file = match greeting_file_result {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.rs") {
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("Problem creating the file : {:?}", e),
//     //         },
//     //         other_error => {
//     //             panic!("Problem opening the file: {:?}", other_error);
//     //         }
//     //     },
//     // };

//     // alternate way

//     // using `unwrap_or_else`
//     // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error| {
//     //             panic!("Problem creating the file: {:?}", error);
//     //         })
//     //     } else {
//     //         panic!("Problem opening the file: {:?}", error);
//     //     }
//     // });

//     // using `unwrap` function
//     // let greeting_file2 = File::open("hello.txt").unwrap();

//     // using `expect` method
//     let greeting_file =
//         File::open("hello.txt").expect("hello.txt should be included in this project");
// }

use std::io::{self, Read};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/////////////////////////////////////
/// The `?` Operator ///////////////
///////////////////////////////////

fn read_username_from_file_qm() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username);
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.line().next()?.chars().last()
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value : i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {value}
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}