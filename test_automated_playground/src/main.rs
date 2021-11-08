#[cfg(test)]
/*
To change a function into a test function, add #[test] on the line before fn. When you run your tests with the cargo test command, Rust builds a test runner binary that runs the functions annotated with the test attribute and reports on whether each test function passes or fails.
*/
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

fn main() {
    println!("test!");
}
