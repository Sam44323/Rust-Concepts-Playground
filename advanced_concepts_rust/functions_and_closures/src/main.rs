// example of passing functions as arguments to another function

fn function_pointers() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_thrice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg) + f(arg)
    }

    let answer = do_thrice(add_one, 5);
    println!("The answer is {}", answer);
}

fn closure_returning() {
    // this means this function will return anything that implements the Fn trait that takes an i32 and returns an i32
    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }
}

fn main() {
    println!("Hello, world!");
    function_pointers();
    closure_returning();
}
