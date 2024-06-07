use std::env;
use std::fs;

fn main() {
    println!("Hello, grep!");

    let args: Vec<String> = env::args().collect();
    // Index "0" is binary basics -> "target/debug/grep"
    // Command: `cargo run bog poem.txt`
    let query = &args[1]; // Something
    let filename = &args[2]; // poem.txt

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file...");

    println!("Content {}", contents);
}
