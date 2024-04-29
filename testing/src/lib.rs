#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    #[allow(dead_code)]
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            )
        }

        Guess { value }
    }
}

pub fn prints_and_returns(a: i32) -> i32 {
    println!("I got value {}", a);
    a
}

pub fn unit_adder(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // Equal
        assert_eq!(4, add_two(2));
        // Not Equal
        assert_ne!(4, add_two(3));
    }

    #[test]
    // Delete #[ignore] to run this test as it is test fail example
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Niferu");

        // Adding custom message on failed test
        assert!(
            result.contains("Niferu"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Panic for only specific message
    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    // To get this working we need to run test with following command: 'cargo test -- --show-output'
    #[test]
    fn it_prints() {
        let value = prints_and_returns(777);

        assert_eq!(777, value);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run...
    }

    #[test]
    fn internal() {
        // Even if internal adder is private we can still call it here
        assert_eq!(4, internal_adder(2, 2))
    }
}
