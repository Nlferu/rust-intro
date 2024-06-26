use std::env;
use std::process;

use grep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // Try running: cargo run > output.txt | cargo run to poem.txt > output.txt
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
