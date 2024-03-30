// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub fn median() {
    let mut data = vec![3, -5, 8, 66, 4, -777, -33, 7, 9, 0, 88, -11];

    data.sort();
    // Return median & mode

    println!("Sorted Data: {:?}", data);
}
