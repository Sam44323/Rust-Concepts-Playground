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

fn main() {
    println!("Hello, world!");
    function_pointers();
}
