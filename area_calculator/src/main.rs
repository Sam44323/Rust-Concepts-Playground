#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };

    let another_rect: Rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    println!("Rectangle-first is: {:#?}", rect);
    println!("Rectangle-another is: {:#?}", another_rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("Can rect hold rectangle {}", rect.can_hold(&another_rect));
}
