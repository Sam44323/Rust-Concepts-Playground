fn iterator_example() {
    let v1 = vec![1, 2, 3];

    for val in v1 {
        println!("Got: {}", val);
    }
}

fn main() {
    iterator_example();
    println!("Hello, world!");
}
