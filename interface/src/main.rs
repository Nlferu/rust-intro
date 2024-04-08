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

        if let Some(department) = company.departments.get_mut(department_name) {
            department.employees.push(employee_name.clone());

            println!(
                "Employee '{}' added to '{}' department!",
                employee_name, department_name
            )
        } else {
            println!("Error: Department '{}' does not exist!", department_name);
        }
    }

    fn remove_employee(&self, employee_name: String) {
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
    fn add_department(&mut self, department: String) {
        self.departments
            .insert(department.clone(), Department { employees: vec![] });

        println!("Department '{}' created!", department);
    }

    fn get_employees_in_department(&self, department_name: &String) {
        let input_formatted = match input_formatter(department_name) {
            Ok(name) => name,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        };

        let employees = self
            .departments
            .get(&input_formatted)
            .map(|department| &department.employees);

        let employees = match employees {
            Some(ref e) if e.is_empty() => None,
            _ => employees,
        };

        println!(
            "Employees for '{}' department: {:?}",
            input_formatted, employees
        )
    }

    fn get_all_company_data(&self) {
        for (department, employees) in &self.departments {
            println!("Department '{}': {:?}", department, employees);
        }
    }

    fn department_existance_checker(&self, department_name: &str) -> bool {
        self.departments
            .keys()
            .any(|key| key.to_lowercase() == department_name.to_lowercase())
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
            // TO BE FIXED FOR LOWER CASE DEPARTMENT NAME
            "add" => {
                println!("Enter Department Name: ");
                let department_name = add_parameter();

                if company.department_existance_checker(&department_name) {
                    println!("Enter Employee Full Name: ");
                    let employee_name = add_parameter();

                    department.add_employee(&mut company, &department_name, employee_name);
                } else {
                    println!("Department '{}' does not exist", department_name)
                }
            }
            "remove" => {
                println!("Enter Employee Full Name: ");
                let employee_name = add_parameter();

                department.remove_employee(employee_name)
            }
            "create" => {
                println!("Enter Department Name: ");

                let department_name = add_parameter();

                if company.department_existance_checker(&department_name) {
                    println!("Error: Department '{}' already exists!", department_name)
                } else {
                    company.add_department(department_name);
                }
            }
            "update" => println!("Department Updated!"),
            "department" => {
                println!("Enter Department Name To Get Its Employees: ");

                let department_name = add_parameter();

                company.get_employees_in_department(&department_name)
            }
            "employees" => department.get_employees(),
            "company" => company.get_all_company_data(),
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

fn add_parameter() -> String {
    let mut parameter: String = String::new();

    io::stdin()
        .read_line(&mut parameter)
        .expect("Failed to read line");

    parameter.trim().to_string()
}

fn input_formatter(input: &String) -> Result<String, &'static str> {
    if input.is_empty() {
        return Err("Input string is empty");
    }

    // Split input by whitespace into words
    let words: Vec<&str> = input.split_whitespace().collect();

    // Capitalize the first letter of each word and collect into a new vector
    let formatted_words: Vec<String> = words
        .into_iter()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(), // Empty word
                Some(first_char) => {
                    first_char.to_uppercase().collect::<String>() + &chars.collect::<String>()
                }
            }
        })
        .collect();

    // Join the formatted words with spaces to get the final formatted string
    let formatted_string = formatted_words.join(" ");

    Ok(formatted_string)
}
