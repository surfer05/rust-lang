fn main() {
    println!("Hello, world!");
    

    let mut s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s;

    let slice = &s[0..2];
    let slice = &s[..2]; 
    // line 11 and 12 are same

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
    // line 17 and 18 are same

    let slice = &s[0..len];
    let slice = &s[..];
    // line 20 and 21 are same


}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}