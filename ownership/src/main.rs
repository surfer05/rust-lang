use std::rc::Rc;
fn main() {
    println!("Hello World");

    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;

    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}");

    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");

    let mut v3: Vec<String> = vec![String::from("Hello World")];
    let mut s2: String = v3.remove(0);
    s2.push('!');
    println!("{s2}");
    assert!(v3.len() == 0);

    // Mutating Different Tuple Fields
    let mut name = (String::from("Ferris"), String::from("Rustacean"));

    let first = &name.0;

    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    // Mutating Different Array Elements
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");
}

fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

fn return_a_string1() -> &'static str {
    "Hello World"
}

fn return_a_string2() -> Rc<String> {
    let s = Rc::new(String::from("Hello World"));
    Rc::clone(&s)
}

fn return_a_string3(output: &mut String) {
    output.replace_range(.., "Hello world");
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

fn stringify_name_with_title1(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str("Esq.");
    full
}
//This solution works because slice::join already copies the data in name into the string full.

// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }
