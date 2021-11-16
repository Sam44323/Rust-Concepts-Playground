use std::env;

/*
use of std::env::args
-----------------------
To enable minigrep to read the values of command line arguments we pass to it, we’ll need a function provided in Rust’s standard library, which is std::env::args. This function returns an iterator of the command line arguments that were given to minigrep.

Note: regarding iterators:
-------------------------
Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.
*/

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    /*
    Note:
    ------
    As we saw when we printed the vector, the program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
    */
    let query = &cli_args[1];
    let filename = &cli_args[2];
    println!("Searching for {} in {}", query, filename);
}
