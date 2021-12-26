fn box_initializer() {
    let b = Box::new(5);
    println!("Box: {}", b);
}

fn main() {
    box_initializer();
}
