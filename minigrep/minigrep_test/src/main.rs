extern crate minigrep_lib;
use std::env;
use std::process;

use minigrep_lib::grep;

fn main() {
    let args: Vec<String> = env::args().collect(); // for getting and parsing the cli arguments

    let config = grep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // using the Error switch pattern for catching errors while reading the file
    if let Err(e) = grep::file_reader(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
