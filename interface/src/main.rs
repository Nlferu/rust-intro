// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Department {
    employees: Vec<String>,
}

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Department>,
}

fn main() {
    println!("\nCompany Management Interface");
    println!("\nType `help` for available commands");

    let commands = [
        "add",        // add new employee
        "remove",     // remove employee
        "create",     // create new department
        "department", // shows all company employees for given department
        "employees",  // shows all company employees and their departments
        "help",       // gives all available commands
        "exit",       // exits program
    ];
    let departments: Vec<&str> = vec!["Engineering", "Sales", "Programming", "Design"];

    loop {
        let mut user_command: String = String::new();

        io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");

        match user_command.trim().to_lowercase().as_str() {
            "add" => println!("New Employee Added!"),
            "remove" => println!("Employee Removed!"),
            "create" => println!("New Department Created!"),
            "department" => println!("All company employees for this department:"),
            "employees" => println!("All employees:"),
            "help" => help(),
            "exit" => {
                println!();
                return;
            }
            _ => {
                println!("Please type proper command, use `help` for all available options");
                continue;
            }
        }
    }
}

fn add(department: String, employee: String) {}

fn remove(department: &str, employee: &str) {}

fn add_department(department: String) {}

fn get_employees_in_department(department: &str) {}

fn get_all_employees(structure: &mut HashMap<&str, &str>) {
    for (department, employee) in structure {}
}

fn help() {
    println!("Company Management Interface - Helper");
}
