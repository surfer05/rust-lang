use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /// Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // `get` method returns an Option<&V>;, no value for that key -> `get` returns `None`
    // handling `Option` by calling `copied` to get an `Option<i32>` rather than an `Option<&i32>`
    // then `unwrap_or` to set `score` to zero if `scores` doesn't have an entry for the key.

    //////// UPDATING A HASHMAP /////////////////

    /// Adding a Key and Value Only If a Key Isn't Present
    scores.entry(String::from("Yellow")).or_insert(70);
    scores.entry(String::from("Red")).or_insert(97);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /// Updating a Valud Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }

    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // reading elements of a vector
    let third: &i32 = &v2[2]; // & and [] gives a reference to the element at the index value
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterating over the values in a vector
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly;
    let s = "initial contents".to_string();
    let mut a = String::from("foo");
    a.push_str("bar");

    let mut lo = String::from("lo");
    lo.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;

    let sa = String::from("hello");
    // let h = sa[0]; // this line will give an error. the type `String` cannot be indexed by `{integer}`

    let hello = String::from("Hola"); // len will be 4 -> Hola is 4 bytes long

    let hello = String::from("नमस्ते");

    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
    for c in hello.chars() {
        // println!("{c}");
    }

    for c in hello.bytes() {
        // println!("{c}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}");
}
