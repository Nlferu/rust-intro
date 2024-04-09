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
    fn add_employee(
        &mut self,
        company: &mut Company,
        department_name: &str,
        employee_name: String,
    ) {
        self.employees.push(employee_name.clone());

        match company.departments.get_mut(department_name) {
            Some(department) => {
                department.employees.push(employee_name.clone());
                println!(
                    "Employee '{}' added to '{}' department!",
                    employee_name, department_name
                );
            }
            None => println!("Error: Department '{}' does not exist!", department_name),
        }
    }

    fn remove_employee(&self, employee_name: &str) {
        println!("Employee '{}' removed!", employee_name)
    }

    fn get_employees(&self) {
        println!("Employees: {:?}", self.employees)
    }
}

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Department>,
}

impl Company {
    // Get rid of department_exists or consolidate it
    fn add_department(&mut self, department_name: String) {
        if self.departments.contains_key(&department_name) {
            println!("Error: Department '{}' already exists!", department_name);
            return;
        }

        self.departments
            .insert(department_name.clone(), Department { employees: vec![] });
        println!("Department '{}' created!", department_name);
    }

    fn get_employees_in_department(&self, department_name: &str) {
        match self.departments.get(department_name) {
            Some(department) => {
                if department.employees.is_empty() {
                    println!("No employees in '{}' department.", department_name);
                } else {
                    println!(
                        "Employees in '{}' department: {:?}",
                        department_name, department.employees
                    );
                }
            }
            None => println!("Error: Department '{}' does not exist!", department_name),
        }
    }

    fn get_whole_company_data(&self) {
        for (department, employees) in &self.departments {
            println!("Department '{}': {:?}", department, employees);
        }
    }
}

fn main() {
    println!("\nCompany Management Interface");
    println!("\nType `help` for available commands");

    // To be moved to initial departments
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
            // Change below as "department"
            "add" => {
                println!("Enter Department Name:");

                if let Ok(department_name) = add_parameter() {
                    match company.departments.get(&department_name) {
                        Some(_) => {
                            println!("Enter Employee Full Name: ");
                            if let Ok(employee_name) = add_parameter() {
                                department.add_employee(
                                    &mut company,
                                    &department_name,
                                    employee_name,
                                );
                            } else {
                                println!("Error: Failed to get employee name!");
                            }
                        }
                        None => println!("Error: Department '{}' does not exist!", department_name),
                    }
                } else {
                    println!("Error: Failed to get department name!")
                }
            }
            "remove" => {
                println!("Enter Employee Full Name: ");

                if let Ok(employee_name) = add_parameter() {
                    department.remove_employee(&employee_name);
                } else {
                    println!("Error: Failed to get employee name!");
                }
            }
            "create" => {
                println!("Enter Department Name: ");

                if let Ok(department_name) = add_parameter() {
                    company.add_department(department_name);
                } else {
                    println!("Error: Failed to get department name!")
                }
            }
            "update" => println!("Department Updated!"),
            "department" => {
                println!("Enter Department Name To Get Its Employees: ");

                if let Ok(department_name) = add_parameter() {
                    company.get_employees_in_department(&department_name);
                } else {
                    println!("Error: Failed to get department name!")
                }
            }
            "employees" => department.get_employees(),
            "company" => company.get_whole_company_data(),
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
        "company - shows whole company structure with employees and their departments",
        "help - gives all available commands",
        "exit - exits program",
    ];

    for item in commands {
        println!("{}", item)
    }
}

fn add_parameter() -> Result<String, String> {
    let mut parameter = String::new();

    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to read line");

    let parameter = parameter.trim().to_string();

    if parameter.is_empty() {
        return Err(format!("Error: Parameter cannot be empty"));
    }

    input_formatter(&parameter)
}

fn input_formatter(input: &String) -> Result<String, String> {
    if input.is_empty() {
        return Err(format!("Error: Input string is empty!"));
    }

    // Split input by whitespace into words
    let words: Vec<&str> = input.split_whitespace().collect();

    // Capitalize the first letter of each word and collect into a new vector
    let formatted_words: Vec<String> = words
        .into_iter()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                // Empty word
                None => String::new(),
                Some(first_char) => {
                    first_char.to_uppercase().collect::<String>()
                        + &chars.collect::<String>().to_lowercase()
                }
            }
        })
        .collect();

    // Join the formatted words with spaces to get the final formatted string
    let formatted_string = formatted_words.join(" ");

    Ok(formatted_string)
}
