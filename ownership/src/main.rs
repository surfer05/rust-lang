fn main() {
    println!("Hello, world!");

    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}


fn add_suffix(mut name: String) -> String {
    name.push_str("Jr.");
    name
}