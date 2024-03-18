use rand::Rng;
use std::io;

fn main() {
    println!("Guess The Number!");

    // (1..=100) `=` means that we include 100 in our scope too
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

    // This line is creating a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess: String = String::new();

    io::stdin()
        // It gives us anything that user enter as string
        .read_line(&mut guess)
        // It checks if input was `Ok` -> returns value or `Err` -> returns error message
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
