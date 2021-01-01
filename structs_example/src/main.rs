#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 50,
        height: 80
    };

    let rect2 = Rectangle {
        width: 10,
        height: 30
    };

    let rect3 = Rectangle {
        width: 70,
        height: 90
    };

    let square = Rectangle::square(100);

    println!(
        "The area of the square is {} square pixels",
        square.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // let rect1 = (30, 50);

    // let width1 = 30;
    // let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        // area(&rect1)
        // area(rect1)
        // area(width1, height1)
    );

    println!("rect is {:?}", rect1);
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
