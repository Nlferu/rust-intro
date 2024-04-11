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
    fn add_employee(&mut self, company: &mut Company) {
        println!("Enter Department Name:");

        if let Ok(department_name) = add_parameter() {
            if let Some(department) = company.departments.get_mut(&department_name) {
                println!("Enter Employee Full Name:");

                if let Ok(employee_name) = add_parameter() {
                    // Prevent adding multiple employees with same name
                    if self.employees.iter().any(|item| item == &employee_name) {
                        println!("Employee with name '{}' currently exists!", employee_name);
                        println!("Consider adding employee identifier to add this employee");
                        return;
                    }

                    // Adding new employee to Department.employees vector
                    self.employees.push(employee_name.clone());
                    // Adding new employee to exact company department
                    department.employees.push(employee_name.clone());

                    println!(
                        "Employee '{}' added to '{}' department!",
                        employee_name, department_name
                    );
                } else {
                    println!("Error: Failed to get employee name!")
                }
            } else {
                println!("Error: Department '{}' does not exist!", department_name);
            }
        } else {
            println!("Error: Failed to get department name!")
        }
    }

    // To be implemented
    fn remove_employee(&mut self, company: &mut Company) {
        println!("Enter Employee Full Name To Remove: ");

        if let Ok(employee_name) = add_parameter() {
            // Removing from Department's employees vector
            if let Some(index) = self.employees.iter().position(|e| e == &employee_name) {
                self.employees.remove(index);
            }

            // Removing from struct
            for department in company.departments.values_mut() {
                if let Some(index) = department
                    .employees
                    .iter()
                    .position(|e| e == &employee_name)
                {
                    department.employees.remove(index);
                    println!("Employee '{}' removed!", employee_name);
                    return;
                }
            }

            println!("Employee '{}' not found in any department.", employee_name);
        } else {
            println!("Error: Failed to get employee name!")
        }
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
    fn add_department(&mut self) {
        println!("Enter Department Name: ");

        if let Ok(department_name) = add_parameter() {
            match self.departments.get(&department_name) {
                Some(_) => {
                    println!("Error: Department '{}' already exists!", department_name);
                }
                None => {
                    self.departments
                        .insert(department_name.clone(), Department { employees: vec![] });

                    println!("Department '{}' created!", department_name);
                }
            }
        } else {
            println!("Error: Failed to get department name!")
        }
    }

    // Purpose of changed code
    // fn add_employee(&mut self, company: &mut Company) {
    //     println!("Enter Department Name:");

    //     if let Ok(department_name) = add_parameter() {
    //         if let Some(department) = company.departments.get_mut(&department_name) {
    //             println!("Enter Employee Full Name:");

    //             if let Ok(employee_name) = add_parameter() {
    //                 let employee_name_owned = employee_name; // Take ownership here, no cloning yet.

    //                 // Adding new employee to Department.employees vector
    //                 // This requires cloning because we need to keep the name in both places.
    //                 department.employees.push(employee_name_owned.clone());

    //                 // Since self is a mutable reference to the original struct, we can add without cloning
    //                 // as employee_name_owned is owned within this scope now.
    //                 self.employees.push(employee_name_owned);

    //                 println!(
    //                     "Employee '{}' added to '{}' department!",
    //                     department.employees.last().unwrap(), // This avoids using the name directly
    //                     department_name
    //                 );
    //             } else {
    //                 println!("Error: Failed to get employee name!");
    //             }
    //         } else {
    //             println!("Error: Department '{}' does not exist!", department_name);
    //         }
    //     } else {
    //         println!("Error: Failed to get department name!");
    //     }
    // }

    fn update_department(&mut self) {
        println!("Enter Department Name To Update: ");

        if let Ok(department_name) = add_parameter() {
            match self.departments.get(&department_name) {
                Some(_) => {
                    println!("Enter New Department Name: ");

                    if let Ok(new_department_name) = add_parameter() {
                        if let Some(department) = self.departments.remove(&department_name) {
                            self.departments
                                .insert(new_department_name.clone(), department);

                            println!(
                                "Department '{}' updated to '{}'",
                                department_name, new_department_name
                            );
                        }
                    } else {
                        println!("Error: Failed to get department name!")
                    }
                }
                None => {
                    println!("Error: Department '{}' does not exist!", department_name);
                }
            }
        } else {
            println!("Error: Failed to get department name!")
        }
    }

    fn get_employees_in_department(&self) {
        println!("Enter Department Name To Get Its Employees: ");

        if let Ok(department_name) = add_parameter() {
            match self.departments.get(&department_name) {
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
        } else {
            println!("Error: Failed to get department name!")
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
            "add" => department.add_employee(&mut company),
            "remove" => department.remove_employee(&mut company),
            "create" => company.add_department(),
            "update" => company.update_department(),
            "department" => company.get_employees_in_department(),
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
    println!("Company Management Interface - Helper:");

    let commands = [
        "\nadd - adds new employee",
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
