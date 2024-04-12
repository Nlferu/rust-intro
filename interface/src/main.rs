// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::io;

use alternative::alternative;
use original::{input_formatter, original};

pub mod alternative;
pub mod original;

fn main() {
    //

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read command");

    let command = input_formatter(&command);

    match command {
        Some(p) => p,
        None => {
            println!("Please enter valid command");
            continue;
        }
    };
    alternative();
    original();
}
