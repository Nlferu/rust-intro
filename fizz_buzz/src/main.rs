// Fizz buzz is a group word game. Players take turns to count incrementally.

// Rules:
// Replacing any number divisible by three with the word "fizz" --- if the sum of all digits of that number is divisible by 3
// Number divisible by five with the word "buzz" ---  if the digit on the units place, that is, the last digit of a given number is 5 or 0
// Number divisible by both three and five with the word "fizzbuzz" --- apply both rules

fn main() {
    println!("Hello, world!");
}

// It should return bool -> which tells if given number is divisible by 3
fn fizzer(number: i32) -> bool {
    true
}

// It should return bool -> which tells if given number is divisible by 5
fn buzzer(number: i32) -> bool {
    false
}
