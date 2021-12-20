use std::thread;
use std::time::Duration;

/*
 * Important:
 * Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature,  their types are still considered different
 */

// example of declaring a struct with closures
/**
 * The Cacher struct has a calculation field of the generic type T. The trait bounds on T specify that it’s a closure by using the Fn trait. Any closure we want to store in the calculation field must have one u32 parameter (specified within the parentheses after Fn) and must return a u32
 *
 * The value field is of type Option<u32>. Before we execute the closure, value will be None. When code using a Cacher asks for the result of the closure, the Cacher will execute the closure at that time and store the result within a Some variant in the value field. Then if the code asks for the result of the closure again, instead of executing the closure again, the Cacher will return the result held in the Some variant.
 */

/**
* Meaning of different types of function traits:
- FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.

- FnMut can change the environment because it mutably borrows values.

- Fn borrows values from the environment immutably.
*/

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    /*
    Example of creating a closure function and storing it in a variable.
    Method for defining a closure
    -----------------------------
    - The closure definition comes after the = to assign it to the variable expensive_closure.To define a closure,
      we start with a pair of vertical pipes (|), inside which we specify the parameters to the closure

    - let statement means expensive_closure contains the definition of an anonymous function, not the resulting value  of calling the anonymous function. Recall that we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in expensive_closure.
     */
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
