// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

pub use median::{median, MedianResult};
pub use mode::mode;

mod median;
mod mode;

pub fn median_and_mode(data: Vec<i32>) -> (MedianResult, i32) {
    return (median(data.clone()), mode(&data));
}
