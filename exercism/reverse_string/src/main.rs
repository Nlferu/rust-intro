use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let input: &str = "Hello, world!";
    let reversed: String = reverse_string(input);

    let input_two: &str = "uüu";
    let reversed_two: String = reverse(input_two);
    let rev_three: String = reverse(input);

    println!("Original: {}", input);
    println!("Reversed with reverse_string(): {}", reversed);

    println!("Original: {}", input_two);
    println!("Reversed with reverse(): {}", reversed_two);

    println!("Reversed with reverse(): {}", rev_three);
}

// Below is working only for pure strings
fn reverse_string(input: &str) -> String {
    // Convert the input string to a mutable vector of characters
    let mut chars: Vec<char> = input.chars().collect();

    // Reverse the vector of characters
    chars.reverse();

    // Convert the vector of characters back to a string
    chars.into_iter().collect()
}

// Simpler solution
fn reverse_string_alt(input: &str) -> String {
    input.chars().rev().collect()
}

// Below function is working for `chars` and `String``
fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
