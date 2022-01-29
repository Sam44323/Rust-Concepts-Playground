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

    // if you're returning based on internal closure type(multi closure options), the same Fn return won't work but we need a box wrapper so that it's same

    fn multi_closure_checker(a: i32) -> Box<dyn Fn(i32) -> i32> {
        if a > 0 {
            Box::new(move |b| b + a)
        } else {
            Box::new(move |b| b - a)
        }
    }
}

fn main() {
    println!("Hello, world!");
    function_pointers();
    closure_returning();
}
