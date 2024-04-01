// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn median(mut data: Vec<i32>) {
    let median: f32;

    // Return median & mode
    data.sort();

    println!("Sorted Data: {:?}", data);

    let median_index: usize = data.len() / 2;
    let even_odd_checker: usize = data.len() % 2;

    match even_odd_checker {
        0 => {
            println!("Even Dataset Received");
            median = (data[median_index] as f32 + data[median_index - 1] as f32) / 2.0;
            println!("Median Value Is: {}", median);
        }
        _ => {
            println!("Odd Dataset Received");
            median = data[median_index] as f32;
            println!("Median Value Is: {}", median);
        }
    }
}
