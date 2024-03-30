// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn median() {
    // Return median & mode
    let _even_data = vec![3, -5, 8, 66, 4, -777, -33, 7, 9, 0, 88, -11];
    let odd_data = vec![3, -5, 8, 66, 4, -777, -33, 7, 9, 0, 88, -11, 99];

    let mut data = odd_data;

    data.sort();

    println!("Sorted Data: {:?}", data);

    let even_median_index: usize;
    even_median_index = data.len() / 2;

    let odd_median_index: usize;
    odd_median_index = data.len() / 2 - 1;

    let fin = data[even_median_index];
    println!("Even: {}", fin);

    let calculated = data.len() % 2;
    let median: i32;

    match calculated {
        0 => {
            println!("Even Dataset Received");
            median = (data[even_median_index] + data[odd_median_index]) / 2;
            println!("Median Value Is: {}", median);
        }
        _ => {
            println!("Odd Dataset Received");
            median = data[even_median_index];
            println!("Median Value Is: {}", median);
        }
    }
}
