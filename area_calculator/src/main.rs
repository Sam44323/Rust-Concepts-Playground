#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect: Rectangle = Rectangle {
        width: 30,
        height: 40,
    };

    println!("Rectangle is: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
