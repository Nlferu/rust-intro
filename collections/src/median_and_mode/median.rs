// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)

#[derive(Debug)]
pub enum MedianResult {
    Float(f32),
    Integer(i32),
}

pub fn median(mut data: Vec<i32>) -> MedianResult {
    data.sort();

    println!("Sorted Data: {:?}", data);

    let median_index: usize = data.len() / 2;
    let even_odd_checker: usize = data.len() % 2;

    match even_odd_checker {
        0 => {
            println!("Even Dataset Received");
            let median: f32 = (data[median_index] as f32 + data[median_index - 1] as f32) / 2.0;
            MedianResult::Float(median)
        }
        _ => {
            println!("Odd Dataset Received");
            let median: i32 = data[median_index];
            MedianResult::Integer(median)
        }
    }
}
