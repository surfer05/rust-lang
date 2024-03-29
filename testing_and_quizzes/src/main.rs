fn main() {
    println!("Hello, world!");

    let opt = Some(String::from("Rust"));
    get_or_default(&opt);
}
fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}

fn get_or_default(arg: &Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.clone()
}