// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::collections::HashSet;

pub fn converter(data: &[&str]) -> Vec<String> {
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    let mut converted_words = Vec::new();

    for word in data {
        let mut chars = word.chars();

        let converted_word = match chars.next() {
            Some(char) => {
                let lowercase_char = char.to_ascii_lowercase();
                let rest_of_word: String = chars.collect();

                if vowels.contains(&lowercase_char) {
                    format!("{}-hay", word)
                } else {
                    format!("{}-{}ay", rest_of_word, char)
                }
            }
            None => String::from("Empty word"),
        };

        converted_words.push(converted_word);
    }

    converted_words
}
