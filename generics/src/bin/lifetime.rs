fn main() {
    println!("Hello World!!");

    let s: &'static str = "I have a static lifetime.";

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishamel. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_word(s: &str) -> &str {}

// Step 1 and Step 2
fn first_word<'a>(s: &'a str) -> &'a str {}

// Another example
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// Second rule doesn't apply cause more than one input lifetime.
// Third rule doesn't apply `longest` is a function rather a method.
// After three rules, can't figure out what the return type's lifetime is.

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announe_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please : {}", announcement);
        self.part
    }
}

// using the rules
// first : two parameters, both are given different lifetimes
// second : not valid as two parameters
// third : output given the same lifetime as self, thus all lifetimes have been accounted for.
