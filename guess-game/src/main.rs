use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

fn main() {
    // Making some spaces in console logs
    println!("\nWelcome In Guess The Number Game!\n");
    println!("Type 'quit' at any time to exit game\n");

    println!("Please pick difficulty from following: easy, medium, hard");

    let difficulty: Difficulty = loop {
        // Below is `String` type and unknown size, which means we store it on heap instead of stack
        let mut difficulty_choice: String = String::new();

        io::stdin()
            .read_line(&mut difficulty_choice)
            .expect("Failed to read line");

        // Trim removes all whitespaces
        // as_str() converts data into `String Slice`` in this case from String
        let difficulty: Difficulty = match difficulty_choice.trim().to_lowercase().as_str() {
            // => is match arm operator indicating what action to take if the pattern is matched.
            "easy" => {
                println!("Easy difficulty chosen");
                Difficulty::Easy
            }
            "medium" => {
                println!("Medium difficulty chosen");
                Difficulty::Medium
            }
            "hard" => {
                println!("Hard difficulty chosen");
                Difficulty::Hard
            }
            "quit" => {
                println!("You have quit the game");
                return;
            }
            _ => {
                println!("Please pick one of the following difficulties: easy, medium, hard");
                continue;
            }
        };

        break difficulty;
    };

    let available_guesses: u8 = difficulty_checker(&difficulty);
    println!("You have total of {available_guesses} available guesses");

    // (1..=100) `=` means that we include 100 in our scope too
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut guesses_left: u8 = available_guesses;

    while guesses_left > 0 {
        println!("Please input your guess: ");

        // This line is creating a mutable variable that is currently bound to a new, empty instance of a String
        let mut guess: String = String::new();

        io::stdin()
            // It gives us anything that user enter as string
            .read_line(&mut guess)
            // It checks if input was `Ok` -> returns value or `Err` -> returns error message
            .expect("Failed to read line");

        // Adding quit option
        if guess.trim().to_lowercase() == "quit" {
            println!("You have quit the game");
            return;
        }

        // Modified error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numbers allowed!");
                continue;
            }
        };

        // Instead of below:
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guesses_left -= 1;
        println!("You have {} guesses left...", guesses_left);

        if guesses_left == 0 {
            println!("Out of guesses! The secret number was {}.", secret_number);
        }
    }
}

// We are only reading data, so passing by reference is better and more efficient here
fn difficulty_checker(difficulty: &Difficulty) -> u8 {
    match difficulty {
        Difficulty::Easy => 20,
        Difficulty::Medium => 10,
        Difficulty::Hard => 5,
    }
}
