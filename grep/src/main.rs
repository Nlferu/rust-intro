use std::env;
use std::fs;

fn main() {
    println!("Hello, grep!");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file...");

    println!("Content {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // We are currently cloning to avoid taking ownership of args. We can refactor this using 'lifetimes' to handle this more efficient
    // It is 'new' = 'constructor'
    fn new(args: &[String]) -> Config {
        // Index "0" is binary basics -> "target/debug/grep"
        // Command: `cargo run bog poem.txt`
        let query = args[1].clone(); // Something
        let filename = args[2].clone(); // poem.txt

        Config { query, filename }
    }
}
