use std::fs::File;

fn main() {
    // Rust Backtracer example
    // backtracer();

    // Proper way of handling recoverable errors
    // This has 2 cases: 'success' and 'error'
    // Similar to Option<T>: 'value' and 'none'
    enum _Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");

    match f {
        Ok(f) => println!("Success! {f:?}"),
        Err(error) => println!("Error: {error}"),
    }
}

// To check which exact function caused error we can run below: RUST_BACKTRACE=1 cargo run
#[allow(dead_code)]
fn backtracer() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(22)
}

fn c(num: i32) {
    if num == 22 {
        // Below will handle crash of our program and print following error message
        // Unrecoverable error
        panic!("Don't pass in 22!")
    }
}
