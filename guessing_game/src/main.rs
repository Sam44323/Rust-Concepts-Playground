use rand::Rng;
use std::io;

fn main() {
    let mut guess: String = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101); // high is exclusive

    println!("Guess the number!");
    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess) // concatenating the buffer_data to the guess variable from the stdin
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
