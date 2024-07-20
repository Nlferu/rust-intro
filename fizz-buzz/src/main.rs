// Write a program that prints the number from 1 to 100 and replace each number accordingly.

// Rules:
// Replacing any number divisible by three with the word "fizz" --- if the sum of all digits of that number is divisible by 3
// Number divisible by five with the word "buzz" ---  if the digit on the units place, that is, the last digit of a given number is 5 or 0
// Number divisible by both three and five with the word "fizzbuzz" --- apply both rules

fn main() {
    for i in 1..=100 {
        if fizzer(i) && buzzer(i) {
            println!("fizzbuzz")
        } else if fizzer(i) {
            println!("fizz")
        } else if buzzer(i) {
            println!("buzz")
        } else {
            println!("{i}")
        }
    }
}

// Return bool -> which tells if given number is divisible by 3
fn fizzer(number: i32) -> bool {
    number % 3 == 0
}

// Return bool -> which tells if given number is divisible by 5
fn buzzer(number: i32) -> bool {
    number % 5 == 0
}
