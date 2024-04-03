// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;

fn main() {
    println!("Company Management Interface");
    println!("\nType `help` for available commands");

    let commands = ["add", "remove", "add dep", "department", "employees"];
    let departments: Vec<&str> = vec!["Engineering", "Sales", "Programming", "Design"];
    let mut structure: HashMap<&str, String> = HashMap::new();

    add(structure, "Engineering", employee)
}

fn add(structure: &mut HashMap<&str, &str>, department: String, employee: String) {
    structure.insert(department, employee);
}

fn remove(department: &str, employee: &str) {}

fn add_department(department: String) {}

fn get_employees_in_department(department: &str) {}

fn get_all_employees(structure: &mut HashMap<&str, &str>) {
    for (department, employee) in structure {}
}

fn help() {
    println!("Company Management Interface - Helper");
}
