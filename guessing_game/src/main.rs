// Import the io module from the 'std' lib
use rand::Rng;
use std::io;

// This is a special function which runs at the start of the program
fn main() {
    println!("Guess the numebr!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {secret_number}");

    println!("Please input your guess:");

    // Create a mutable variable called guess, which is a string
    let mut guess = String::new();

    // Standart input from terminal
    io::stdin()
        .read_line(&mut guess) // Read the line in the terminal
        .expect("Failed to read line"); // Handle a situation where the function fails and output an error messege of 'failed to read line'

    println!("You guessed: {guess}"); // Print out the guess, crab pincers '{}' open an expression
}
