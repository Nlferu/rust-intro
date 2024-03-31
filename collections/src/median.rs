// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn median(mut data: Vec<i32>) {
    let median: i32;

    // Return median & mode
    data.sort();

    println!("Sorted Data: {:?}", data);

    let median_index: usize;
    median_index = data.len() / 2;

    let calculated = data.len() % 2;

    match calculated {
        0 => {
            println!("Even Dataset Received");
            median = (data[median_index] + data[median_index - 1]) / 2;
            println!("Median Value Is: {}", median);
        }
        _ => {
            println!("Odd Dataset Received");
            median = data[median_index];
            println!("Median Value Is: {}", median);
        }
    }
}
