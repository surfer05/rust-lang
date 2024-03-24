use std::mem;

fn main() {
    let hello = String::from("Hello, world !!");

    let mut hello = String::from("Hello, ");

    hello.push('w');
    hello.push_str("orld!");

    // some bytes, in a vector
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);

    let s = ['h', 'e', 'l', 'l', 'o'];

    let size: usize = s.into_iter().map(|c| mem::size_of_val(&c)).sum();
    assert_eq!(size, 20);

    let s = "hello";

    let third_character = s.chars().nth(2);
    assert_eq!(third_character, Some('l'));

    let s = "💖💖💖💖💖";
    let third_character = s.chars().nth(2);
    assert_eq!(third_character, Some('💖'));
}
