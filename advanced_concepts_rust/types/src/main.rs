fn type_aliases() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 10;

    println!("x is {} and y is {}", x, y);
}

fn never_type() {
    // never type is denoted by the `!` symbol which means that the function never returns
}

fn main() {
    type_aliases();
    never_type();
}
