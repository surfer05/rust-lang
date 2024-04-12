fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        2 => println!("three"),
        _ => println!("anything"),
    }

    let z = Some(5);
    let y: Option<i32> = None;

    match z {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {y}"),
        _ => println!("Default case, z = {:?}", z),
    }

    println!("at the end : z = {:?} y = {:?}", z, y);

    // Multiple Patterns
    let a = 1;
    match a {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    let b = 5;
    match b {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let ch = 'c';

    match ch {
        'a'..='j' => println!("early ACSII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis : ({x}, {y})");
        }
    }

    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure.");
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {x} and in the y direction {y}");
    //     }
    //     Message::Write(text) => {
    //         println!("Text message: {text}");
    //     }
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change the color to red {r}, green {g}, and blue {b}",)
    //     }
    // }

    let msgg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msgg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    let mssg = Messsage::Hello { id: 5 };

    match mssg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id : {}", id),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Messsage {
    Hello { id: i32 },
}
