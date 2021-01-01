#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    let rect1 = Rectangle {
        width: 50,
        height: 80
    };

    // let rect1 = (30, 50);

    // let width1 = 30;
    // let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        // area(rect1)
        // area(width1, height1)
    );

    println!("rect is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
