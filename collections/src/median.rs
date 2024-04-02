// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn median_and_mode(mut data: Vec<i32>) {
    // Mode
    let mut map = HashMap::new();

    for &number in &data {
        // If key already exist it reads its value and adds 1 to it.
        // If key doesnt exist it sets it to 1
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max: i32 = 0;
    let mut mode: i32 = 0;

    for (key, value) in map {
        if value > max {
            max = value;
            mode = key;
        }
    }

    // Median
    data.sort();

    println!("Sorted Data: {:?}", data);

    let median_index: usize = data.len() / 2;
    let even_odd_checker: usize = data.len() % 2;

    match even_odd_checker {
        0 => {
            println!("Even Dataset Received");
            let median: f32 = (data[median_index] as f32 + data[median_index - 1] as f32) / 2.0;

            println!("Median Value Is: {} Mode Value Is: {:?}", median, mode);
        }
        _ => {
            println!("Odd Dataset Received");
            let median: i32 = data[median_index];
            println!("Median Value Is: {} Mode Value Is: {:?}", median, mode);
        }
    }
}
