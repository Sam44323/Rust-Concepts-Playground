struct Point {
    x: i32,
    y: i32,
}

fn destructure_pattern_matching() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // destructuring the patterns with assigning x to a and y to b
    assert_eq!(0, a);
    assert_eq!(7, b);
}

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

    // ------------------------------------------------
    // Adding range patterns matching
    // ------------------------------------------------

    let x = 6;

    match x {
        1..=10 => println!("one through ten"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early letter"),
        'k'..='z' => println!("late letter"),
        _ => println!("This is something else"),
    }

    destructure_pattern_matching();
}
