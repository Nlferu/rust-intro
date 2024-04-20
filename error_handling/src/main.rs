use std::fs::File;
use std::io::ErrorKind;

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

    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => {
            println!("Success! {file:?}");
            file
        }
        // We can here either use 'println!' macro or 'panic!'
        // Err(error) => panic!("Error: {error:?}") || Err(error) => println!("Error: {error:?}")

        // Instead of panicking we can handle things on error like below:
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(file_created) => file_created,
                Err(e) => panic!("Problem while creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem while opening the file: {:?}", other_error)
            }
        },
    };

    println!("File Creation Handled {:?}", f);

    // Another way to write above code using 'CLOSURES'
    let f = File::open("file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("file.txt").unwrap_or_else(|error| {
                panic!("Problem while creating the file: {error:?}");
            })
        } else {
            panic!("Problem while opening the file: {:?}", error)
        }
    });

    println!("File Creation Handled {:?}", f);

    // Instead of using 'panic!' we can use 'unwrap()'
    let f = File::open("non_existent_file.txt").unwrap();

    println!("Message from f: {f:?}");

    // We can specify error message by adding '.expect()'
    let _f = File::open("non_existent_file.txt").expect("Failed to open non_existent_file.txt");

    // Instead of using below match we can add '.unwrap()' to above
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("Program crashed: {err:?}"),
    // };
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
