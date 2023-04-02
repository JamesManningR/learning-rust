// Import the io module from the 'std' lib
use rand::Rng;
use std::io;
use std::cmp::Ordering;

// This is a special function which runs at the start of the program
fn main() {
    println!("Guess the numebr!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {secret_number}");

    loop {
        println!("Please input your guess:");

        // Create a mutable variable called guess, which is a string
        let mut guess = String::new();

        // Standart input from terminal
        io::stdin()
            .read_line(&mut guess) // Read the line in the terminal
            .expect("Failed to read line"); // Handle a situation where the function fails and output an error messege of 'failed to read line'

        let guess: u32 = guess // Create a new variable called guess (Shadowing)
            .trim() // Trim the originals whitespace
            .parse() // Parse into a number
            .expect("Please type a number"); // If it's not a number, then it's invalid

        println!("You guessed: {guess}"); // Print out the guess, crab pincers '{}' open an expression

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Just Right")
                break;
            },
        }
    }
}
