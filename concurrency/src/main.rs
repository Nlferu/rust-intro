use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // If main thread ends -> spawn thread ends too no matter what
    for i in 1..5 {
        println!("Number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
