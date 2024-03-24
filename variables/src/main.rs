use std::io;

fn main1() {
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

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is : {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("The result is {result}");

    for number in (1..4).rev() {
        println!("{number}!");
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is {element}");
    }
}

fn another_function(x: i32) {
    println!("Another function with value: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is : {value}{unit_label}");
}

fn nth_fibonacci_number(n: i32) -> i32 {
    if n <= 0 || n == 1 {
        return n;
    } else {
        return nth_fibonacci_number(n - 1) + nth_fibonacci_number(n - 2);
    }
}

fn main() {
    for int in 0..15 {
        let number: i32 = nth_fibonacci_number(int);
        println!("{number}");
    }
    let temp: f32 = f_to_c(100.0);
    println!("{temp}");
}

fn f_to_c(t_in_f: f32) -> f32 {
    return (t_in_f - 32.0) * (5.0 / 9.0);
}
