// Import the io module from the 'std' lib
use rand::Rng;
use std::cmp::Ordering;
use std::io;

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

        // Create a new variable called guess (Shadowing)
        // Trim the originals whitespace
        // Parse into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); // Print out the guess, crab pincers '{}' open an expression

        // Match uses 'arms' to enumerate states
        match guess.cmp(&secret_number) {
            // In this case we compare guess with secret_number (&... means reference secret_number)
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Just Right");
                break; // If the guess is right, we want to break the loop and end the game
            }
        }
    }
}
