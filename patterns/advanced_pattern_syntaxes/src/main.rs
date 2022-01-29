fn main() {
    // ------------------------------------------------
    // Matching the literals
    // ------------------------------------------------

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ------------------------------------------------
    // Named variables matching with printing
    // ------------------------------------------------

    let x = Some(10);

    match x {
        Some(10) => println!("ten"),
        Some(i) => println!("Value {}", i),
        None => println!("none"),
    }
}
