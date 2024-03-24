use std::io;

fn main() {
    println!("Hello, world!");

    let x = 5;
    let x = x + 1;
    {
        let x = x + 2;
        println!("The value of x in the inner scope is : {x}");
    }
    println!("The values of x is : {x}");

    let a: (i32, f64, u8) = (500, 6.4, 1);
    let five_00 = a.0;
    let six_4 = a.1;
    let one = a.2;

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is : {y}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index : usize = index.trim().parse().expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is : {element}");

    another_function(5);
    print_labeled_measurement(10, 'h');
}

fn another_function(x: i32) {
    println!("Another function with value: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}
