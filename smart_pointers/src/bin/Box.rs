use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x); // `y` is an instance of `Box<T>` pointing to a copied value of `x`
    let y = &x; // a reference pointing to the value of `x`

    let p = 6;
    let q = MyBox::new(p);

    assert_eq!(6, p);
    assert_eq!(6, *q);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Defining our Own Smart Pointer
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
