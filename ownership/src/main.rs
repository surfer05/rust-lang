use core::prelude::rust_2015;

fn main() {
    println!("Hello, world!");

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;

    let r1: &Box<i32> = &x;
    let b: i32 = **r1;

    let r2: &i32 = &*x;
    let c: i32 = *r2;

    println!("{} {} {}!", a, b, c);

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);

    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);


    // Buggy code ahead
    // let mut v: Vec<i32> = vec![1,2,3];
    // let num: &i32 = &v[2];
    // v.push(4);
    // println!("Third element is {}", *num);
    // buggy code ends

    let mut v : Vec<i32> = vec![1,2,3];
    let num : &mut i32 = &mut v[2];

    let num2: &i32 = &*num;

    println!("{} {}", *num, *num2);

    let s = String::from("Hello World");
    let s_ref = &s;

    drop(s);
    println!("{}", s_ref);

}

fn add_suffix(mut name: String) -> String {
    name.push_str("Jr.");
    name
}

fn first(strings : &Vec<String>) -> &String {
    let s_ref = &strings[0];
    s_ref
}
