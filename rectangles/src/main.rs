#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        &self.width * &self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let scale = 2;

    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect.area());
    dbg!(&rect);

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    dbg!(rect.can_hold(&rect2));
    dbg!(rect2.can_hold(&rect));

    dbg!(Rectangle::square(10));
}
