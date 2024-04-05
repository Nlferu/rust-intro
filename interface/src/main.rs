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
    fn add_employee(&mut self, employee_name: String) {
        self.employees.push(employee_name);
    }
}

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Department>,
}

impl Company {
    fn add_employee_to_department(&mut self, department_name: &str, employee_name: String) {
        if let Some(department) = self.departments.get_mut(department_name) {
            department.add_employee(employee_name);
        } else {
            println!("Department '{}' does not exist!", department_name);
        }
    }

    fn _remove_employee(&self, _department: &str, _employee: &str) {}

    fn add_department(&mut self, department: String) {
        self.departments
            .insert(department.clone(), Department { employees: vec![] });
        println!("Department '{}' created!", department);
    }

    fn get_employees_in_department(&self) {
        for (department, employees) in &self.departments {
            println!("Department '{}': {:?}", department, employees);
        }
    }

    // This should be returning self?
    fn _get_all_employees() {}
}

fn main() {
    println!("\nCompany Management Interface");
    println!("\nType `help` for available commands");

    let _departments: Vec<&str> = vec!["Engineering", "Sales", "Programming", "Design"];

    let mut department: Department = Department {
        employees: Vec::new(),
    };

    let mut company: Company = Company {
        departments: HashMap::new(),
    };

    loop {
        let mut user_command: String = String::new();

        io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");

        match user_command.trim().to_lowercase().as_str() {
            "add" => {
                println!("Enter Department Name: ");
                let department_name = add_parameter();

                println!("Enter Employee Full Name: ");
                let employee_name = add_parameter();

                company.add_employee_to_department(&department_name, employee_name);
            }
            "remove" => println!("Employee Removed!"),
            "create" => {
                println!("Enter Department Name: ");

                let parameter = add_parameter();

                company.add_department(parameter);
            }
            "update" => println!("Department Updated!"),
            "department" => {
                company.get_employees_in_department();
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

    let commands = [
        "add - adds new employee",
        "remove - removes employee",
        "create - creates new department",
        "update - updates department name",
        "department - shows all company employees for given department",
        "employees - shows all company employees and their departments",
        "help - gives all available commands",
        "exit - exits program",
    ];

    for item in commands {
        println!("{}", item)
    }
}

fn add_parameter() -> String {
    let mut parameter: String = String::new();

    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to read line");

    parameter.trim().to_string()
}
