use std::thread;
use std::time::Duration;

/*
 * Important:
 * Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature,  their types are still considered different
 */

fn generate_workout(intensity: u32, random_number: u32) {
    /*
    Example of creating a closure function and storing it in a variable.
    Method for defining a closure
    -----------------------------
    - The closure definition comes after the = to assign it to the variable expensive_closure.To define a closure,
      we start with a pair of vertical pipes (|), inside which we specify the parameters to the closure

    - let statement means expensive_closure contains the definition of an anonymous function, not the resulting value  of calling the anonymous function. Recall that we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in expensive_closure.
     */

    let expensive_result = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result(intensity));
        println!("Next, do {} situps!", expensive_result(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
