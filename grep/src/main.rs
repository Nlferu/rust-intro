use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    println!("Hello, grep!");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // We are currently cloning to avoid taking ownership of args. We can refactor this using 'lifetimes' to handle this more efficient
    // It is 'new' = 'constructor'
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // Index "0" is binary basics -> "target/debug/grep"
        // Command: `cargo run bog poem.txt`
        let query = args[1].clone(); // Something
        let filename = args[2].clone(); // poem.txt

        Ok(Config { query, filename })
    }
}
