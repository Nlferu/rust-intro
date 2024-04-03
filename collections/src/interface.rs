// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;

pub fn interface() {
    let departments: Vec<&str> = vec!["Engineering", "Sales", "Programming", "Design"];
}

pub fn add(person: &str, department: &str) {}

pub fn remove(person: &str, department: &str) {}

pub fn get_people_per_department(department: &str) {}

pub fn get_all_employees() {}
