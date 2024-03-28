#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area1(rect1)
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let a = area2(rect1);

    println!("{} * {} = {}", rect1.width, rect1.height, a);


    dbg!(&rect1);

    println!("rect1 is {:#?}", rect1)
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: Rectangle) -> u32 {
    rectangle.width* rectangle.height
}