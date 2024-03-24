use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let input: &str = "Hello, world!";
    let reversed_string: String = reverse_string(input);
    let reversed_string_alt: String = reverse_string_alt(input);
    let reversed: String = reverse(input);

    println!("Original: {}", input);
    println!("Reversed with reverse_string(): {}", reversed_string);
    println!(
        "Reversed with reverse_string_alt(): {}",
        reversed_string_alt
    );
    println!("Reversed with reverse(): {}", reversed);

    println!("------------------------------------------------------------");

    let input: &str = "uüu";
    let reversed_string: String = reverse_string(input);
    let reversed_string_alt: String = reverse_string_alt(input);
    let reversed: String = reverse(input);

    println!("Original: {}", input);
    println!("Reversed with reverse_string(): {}", reversed_string);
    println!(
        "Reversed with reverse_string_alt(): {}",
        reversed_string_alt
    );
    println!("Reversed with reverse(): {}", reversed);

    println!("------------------------------------------------------------");

    let input: &str = "畫惡魔";
    let reversed_string: String = reverse_string(input);
    let reversed_string_alt: String = reverse_string_alt(input);
    let reversed: String = reverse(input);

    println!("Original: {}", input);
    println!("Reversed with reverse_string(): {}", reversed_string);
    println!(
        "Reversed with reverse_string_alt(): {}",
        reversed_string_alt
    );
    println!("Reversed with reverse(): {}", reversed);
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
