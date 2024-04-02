// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

use std::collections::HashSet;

pub fn converter(data: &[&str]) {
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();

    for word in data {
        let mut chars = word.chars();

        match &chars.next() {
            Some(char) => {
                let lowercase_char = &char.to_ascii_lowercase();
                if vowels.contains(&lowercase_char) {
                    println!("{}-ay", &word[1..]);
                } else {
                    println!("{}-hay", &word);
                }
            }
            None => println!("Empty word"),
        }
        //println!("Lama: {:?}", chars);
        //println!("Data: {:?}", chars.next().unwrap().to_ascii_lowercase());
    }
}
