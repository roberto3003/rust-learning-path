#[derive(Debug)]
struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn area(&self) -> u16 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u16) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
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

    let rect3 = Rectangle::square(45);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("\nrect1 is {:#?}", rect1);
    println!("\nrect3 is {:#?}", rect3);

    println!("\nCan rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("\nCan rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
