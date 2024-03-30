// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn median() {
    let mut even_data = vec![3, -5, 8, 66, 4, -777, -33, 7, 9, 0, 88, -11];
    let mut odd_data = vec![3, -5, 8, 66, 4, -777, -33, 7, 9, 0, 88, -11, 99];

    even_data.sort();
    odd_data.sort();
    // Return median & mode

    println!("Sorted Data: {:?}", even_data);

    let even_median_index: usize;
    even_median_index = even_data.len() / 2;

    let odd_median_index: usize;
    odd_median_index = odd_data.len() / 2 - 1;

    let fin = even_data[even_median_index];
    println!("Even: {}", fin);

    let calculated = even_data.len() % 2;

    match calculated {
        0 => println!("even"),
        _ => println!("odd"),
    }
}
