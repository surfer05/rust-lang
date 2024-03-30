use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let v = vec![1, 2, 3];
    // v[99];
    let greeting_file_result = File::open("hello.rs");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.rs") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    
}
