// Given a list of integers, use a vector and return mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn mode(data: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for &number in data {
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

    return mode;
}
