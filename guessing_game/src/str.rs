use std::slice;
use std::str;

fn main() {
    let hello_world = "Hello, World!";

    let hello_world: &'static str = "Hello, world !";

    let story = "Once upon a time...";

    let ptr = story.as_ptr();
    let len = story.len();

    println!("{}",len);

    // story has nineteen bytes
    assert_eq!(19, len);

    // Rebuilding a str out of ptr and len.
    let s = unsafe {
        // First, we build a &[u8]....
        let slice = slice::from_raw_parts(ptr, len);

        // .... and then convert that slice into a string value
        str::from_utf8(slice)
    };
}