use std::env;

fn main() {
    println!("Hello, grep!");

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
