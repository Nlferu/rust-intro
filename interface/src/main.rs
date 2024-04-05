// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Department {
    employees: Vec<String>,
}

impl Department {
    fn add_employee(&mut self, name: String) {
        self.employees.push(name.clone());
        println!("Employee {} added!", name);
    }

    fn _remove_employee(&self, _department: &str, _employee: &str) {}
}

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Department>,
}

impl Company {
    fn add_department(&mut self, department: String) {
        self.departments
            .insert(department.clone(), Department { employees: vec![] });
        println!("Department '{}' created!", department);
    }

    fn _get_employees_in_department(_department: &str) {}

    // This should be returning self?
    fn _get_all_employees() {}
}

fn main() {
    println!("\nCompany Management Interface");
    println!("\nType `help` for available commands");

    let _commands = [
        "add",        // add new employee
        "remove",     // remove employee
        "create",     // create new department
        "update",     // updates department name
        "department", // shows all company employees for given department
        "employees",  // shows all company employees and their departments
        "help",       // gives all available commands
        "exit",       // exits program
    ];
    let _departments: Vec<&str> = vec!["Engineering", "Sales", "Programming", "Design"];

    loop {
        let mut user_command: String = String::new();

        let mut department: Department = Department {
            employees: Vec::new(),
        };

        let mut company: Company = Company {
            departments: HashMap::new(),
        };

        io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");

        match user_command.trim().to_lowercase().as_str() {
            "add" => {
                println!("Enter Employee Full Name: ");

                let parameter = add_parameter();

                department.add_employee(parameter);
            }
            "remove" => println!("Employee Removed!"),
            "create" => {
                println!("Enter Department Name: ");

                let parameter = add_parameter();

                company.add_department(parameter);
            }
            "update" => println!("Department Updated!"),
            "department" => {
                println!("All company employees for this department:");
            }
            "employees" => println!("All employees:"),
            "help" => help(),
            "exit" => {
                println!("Company Management Interface Closed!");
                return;
            }
            _ => {
                println!("Please type proper command, use `help` for all available options");
                continue;
            }
        }
    }
}

fn help() {
    println!("Company Management Interface - Helper");
}

fn add_parameter() -> String {
    let mut parameter: String = String::new();

    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to read line");

    parameter.trim().to_string()
}
