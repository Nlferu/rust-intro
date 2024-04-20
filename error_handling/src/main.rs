fn main() {
    // Rust Backtracer example
    // backtracer();

    // Proper way of handling recoverable errors
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

// To check which exact function caused error we can run below: RUST_BACKTRACE=1 cargo run
fn _backtracer() {
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
