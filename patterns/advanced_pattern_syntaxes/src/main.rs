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
        _ => println!("none"),
    }

    // ------------------------------------------------
    // Multi pattern matching (such as using & or |)
    // ------------------------------------------------

    let x = 10;

    match x {
        1 | 2 | 3 | 4 | 5 => println!("one through five"),
        _ => println!("something else"),
    }
}
