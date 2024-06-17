use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Now first outputs will be from spawn thread then we print main thread outputs
    handle.join().unwrap();

    // If main thread ends -> spawn thread ends too no matter what
    for i in 1..5 {
        println!("Number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Now we are getting numbers from spawn thread and main thread simultaneously
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    // We need to move ownership to handle to execute this and let Rust know how long v will live
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
