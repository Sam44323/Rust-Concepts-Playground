use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess: String = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101); // high is exclusive in range

    println!("Guess the number!");

    loop {
        guess.clear();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess) // concatenating the buffer_data to the guess variable from the stdin
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // redeclaring guess to a number
        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // trim removes the whitespace from the string

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "Win!".green());
                break;
            }
        }
    }
}
