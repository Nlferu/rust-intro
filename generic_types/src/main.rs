use std::fmt::Debug;

#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let data = vec![666, 34, 3, 4, 8, 77, 98];
    let chars = vec!['y', 'a', 'z', 'c', 'x'];

    get_max(data);
    get_max(chars);

    let p1 = Point { x: 5, y: 7 };
    let p2 = Point { x: 5.0, y: 7.7 };
    let p3 = Point { x: 5, y: 6.6 };

    println!("p1: {:?} p2: {:?} p3: {:?}", p1, p2, p3);
}

// Universal Function With Generic Types
fn get_max<T: PartialOrd + Copy + Debug>(data: Vec<T>) -> T {
    let mut max = data[0];

    for number in data {
        if number > max {
            max = number;
        }
    }

    println!("Max number is: {max:?}");
    max
}
