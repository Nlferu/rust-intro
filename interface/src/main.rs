// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::io;

use alternative::alternative;
use original::original;

pub mod alternative;
pub mod original;

fn main() {
    println!();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let command = input_formatter(&command);

        match command {
            Some(command) => match command.as_str() {
                "original" => original(),
                "alternative" => alternative(),
                _ => unreachable!(),
            },
            None => println!("Please enter a valid command: 'original' or 'alternative' "),
        };
    }
}

fn input_formatter(input: &String) -> Option<String> {
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "original" | "alternative" => Some(input),
        _ => None,
    }
}
