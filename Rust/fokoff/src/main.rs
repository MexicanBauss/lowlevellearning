#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

pub trait Area {
    fn area(&self) -> i32;
    fn width(&self) -> bool;
    fn can_hold(&self, other: &Rectangle) -> bool;
    fn square(size: i32) -> Self;
}

impl Area for Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}