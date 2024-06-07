use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // We are currently cloning to avoid taking ownership of args. We can refactor this using 'lifetimes' to handle this more efficient
    // It is 'new' = 'constructor'
    pub fn new(args: &[String]) -> Result<Config, &str> {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
