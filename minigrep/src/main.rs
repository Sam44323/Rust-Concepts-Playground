use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
    fn new(args: &[String]) -> Result<Config, &str> {
        /*
        Note:
        ------
        As we saw when we printed the vector, the program’s name takes up the first value in the vector at args[0], so we’re starting at index 1.
        */

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // This will make a full copy of the data for the Config instance to own
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/*
Note:
------
 First, we changed the return type of the run function to Result<(), Box<dyn Error>>. This function previously returned the unit type, (), and we keep that as the value returned in the Ok case.

For the error type, we used the trait object Box<dyn Error>. For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. The dyn keyword is short for “dynamic.”
*/

fn file_reader(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(&config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

    Ok(())
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let config = Config::new(&cli_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = file_reader(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
