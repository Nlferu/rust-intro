use std::{thread, time::Duration};

fn main() {
    // ----------------------------------- Creating Threads -----------------------------------

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

    // ----------------------------------- Message Passing -----------------------------------

    use std::sync::mpsc;

    // Creating channels
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Devil"),
            String::from("does"),
            String::from("not"),
            String::from("sleep"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("his"),
            String::from("name"),
            String::from("is"),
            String::from("Semyazza"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }

    // ----------------------------------- Sharing State -----------------------------------

    use std::rc::Rc;
    use std::sync::Mutex;

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        println!("num = {:?}", num);
        *num = 6;
    }

    println!("m = {:?}", m);

    // Counter
    let counter = Rc::new(Mutex::new(0));

    // Storing all threads
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
