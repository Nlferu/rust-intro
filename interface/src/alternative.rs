// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::{collections::HashMap, io};

pub fn alternative() {
    // This version needs to be improved

    println!("\nCompany Management Interface");

    let mut employee_directory = HashMap::new();

    loop {
        println!("\nEnter a command like \"Add <Person> to <Department>\"");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let command: &str = command.trim();
        println!("You entered following command: {}", command);

        let mut iter = command.split_whitespace();

        // This is getting consumed, so in next nth() we use 1 again
        let person = match iter.nth(1) {
            Some(p) => p,
            None => {
                println!("Please enter valid command");
                continue;
            }
        };

        let department = match iter.nth(1) {
            Some(d) => d,
            None => {
                println!("Please enter valid command");
                continue;
            }
        };

        let employees = employee_directory
            .entry(String::from(department))
            .or_insert(vec![]);
        employees.push(String::from(person));

        println!("Employee directory: {:?}", employee_directory);
    }
}
