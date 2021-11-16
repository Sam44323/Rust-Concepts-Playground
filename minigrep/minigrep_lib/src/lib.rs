use std::error::Error;
use std::fs;

pub struct Config{
    pub query: String,
    pub filename: String,
}

impl Config{
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


pub fn file_reader(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(&config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

    Ok(())
}