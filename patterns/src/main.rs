fn main() {
    println!("Hello, world!");

    let x: Option<i32> = Some(0);

    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // Conditional  if let Expressions

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tueday is green day !");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color ");
        } else {
            println!("Using orange as the backgound color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let mut v = vec![(1, 2), (3, 4)].into_iter();
    let mut sum = 0;
    while let Some(t) = v.next() {
        let (_, n) = t;
        sum += n;
    }
    println!("{sum}");
}
