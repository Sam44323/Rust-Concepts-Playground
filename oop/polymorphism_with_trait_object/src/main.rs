use polymorphism_with_trait_object::*;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw the select box
    }
}

fn main() {
    println!("Hello, world!");
}
