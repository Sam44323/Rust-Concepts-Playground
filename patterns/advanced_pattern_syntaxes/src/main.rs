struct Point {
    x: i32,
    y: i32,
}

enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destructure_pattern_matching() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // destructuring the patterns with mapping x to a and y to b {this is optional BTW, you can keep the variable destructuring as it is}
    assert_eq!(0, a);
    assert_eq!(7, b);

    // ------------------------------------------------
    // Custom pattern destructuring example
    // ------------------------------------------------

    // Structs //

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis at ({}, {})", x, y),
    }

    // Enums //

    let msg = Message::ChangeColor(Color::RGB(0, 0, 0));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("x: {} y: {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(color) => match color {
            Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        },
    }

    // ------------------------------------------------
    // Complex destructuring example
    // ------------------------------------------------

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("Feet: {}, Inches: {}, x: {}, y: {}", feet, inches, x, y);

    // ------------------------------------------------
    // Value ignoring in patterns
    // ------------------------------------------------

    foo(10, 30);

    // ------------------------------------------------
    // Combination pattern matching
    // ------------------------------------------------

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => setting_value = new_setting_value,
    }

    println!("Setting value is: {:?}", setting_value);

    // ------------------------------------------------
    // Un-used value ignorer
    // ------------------------------------------------

    let _x = 10; // if you want to ignore value then use underscore

    // example of ignoring values for structs
    let origin = Point { x: 10, y: 10 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // example of multiple ignorer

    let numbers = (1, 4, 8, 16, 32);

    match numbers {
        (first, .., fifth) => {
            println!("Some numbers: {}, {}", first, fifth);
        }
    }
}

fn foo(_: i32, y: i32) {
    println!("y is {}", y);
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
