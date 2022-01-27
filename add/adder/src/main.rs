use add_nine::add_nine;
use add_one::add_one;

fn main() {
    let mut n = 5;
    println!("Hello! {} plus one is {}", n, add_one(n));

    n = 9;
    println!("Hello! {} plus nine is {}", n, add_nine(n));
}
