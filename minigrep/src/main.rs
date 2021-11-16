use std::env;
use std::fs;

/*
use of std::env::args
-----------------------
To enable minigrep to read the values of command line arguments we pass to it, we’ll need a function provided in Rust’s standard library, which is std::env::args. This function returns an iterator of the command line arguments that were given to minigrep.

Note: regarding iterators:
-------------------------
Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.
*/

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        /*
        Note:
        ------
        As we saw when we printed the vector, the program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
        */

        // This will make a full copy of the data for the Config instance to own
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let config = Config::new(&cli_args);
    // reading the content for the files
    let contents =
        fs::read_to_string(&config.filename).expect("Something went wrong reading the file");
    println!("Searching for {} in {}", config.query, config.filename);
    println!("With text:\n{}", contents);
}
