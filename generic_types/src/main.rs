use std::fmt::Debug;

fn main() {
    let data = vec![666, 34, 3, 4, 8, 77, 98];
    let chars = vec!['y', 'a', 'z', 'c', 'x'];

    get_max(data);
    get_max(chars);
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
