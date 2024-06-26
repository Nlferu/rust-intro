// Iterators in Rust comes from standard library and looks like below:
pub trait IteratorExample {
    type Item;

    // Associated type
    // We need 'mut' as calling next changes the internal state of the iterator used to track where it is in the sequence
    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn iterator_demo() {
    let v = vec![1, 2, 3];

    // IMMUTABLE references 'iter()'
    // To use 'next' we need 'mut' as described in 'Iterator' trait above
    let mut v_iter = v.iter();

    // First out of bound item will be None
    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);

    // MUTABLE references 'iter_mut()'
    // To use 'next' we need 'mut' as described in 'Iterator' trait above
    let mut v = vec![1, 2, 3];

    let mut v_iter = v.iter_mut();

    // First out of bound item will be None
    assert_eq!(v_iter.next(), Some(&mut 1));
    assert_eq!(v_iter.next(), Some(&mut 2));
    assert_eq!(v_iter.next(), Some(&mut 3));
    assert_eq!(v_iter.next(), None);

    // OWN TYPES 'into_iter()'
    let v = vec![1, 2, 3];

    let mut v_iter = v.into_iter();

    // First out of bound item will be None
    assert_eq!(v_iter.next(), Some(1));
    assert_eq!(v_iter.next(), Some(2));
    assert_eq!(v_iter.next(), Some(3));
    assert_eq!(v_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    // We must assign type as there are 2 methods: adapters -> takes iterator and returns iterator || consumers -> takes iterator and returns some other type for example i32
    let total: i32 = v_iter.sum();

    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // Filter returns bool and if it is true item will be included in results and if false item will not be included in results
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    #[allow(dead_code)]
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Own Iterator
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_one_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        // 'zip' method takes 2 iterators and zips them into one iterator containing pair of values
        // (First Iterator: iterator on which zip is called, Second Iterator: iterator passed in zip() method)
        // Second iterator skips 1st element
        .zip(Counter::new().skip(1))
        // Closure, which will call each element from each iterator
        .map(|(a, b)| a * b)
        // Now we filter for items divisible by 3 only
        .filter(|x| x % 3 == 0)
        // Consumer 'sum()' method to sum all results
        .sum();

    assert_eq!(18, sum);
}

fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    for value in v_iter {
        println!("Current Value: {}", value);
    }

    // Adapter & Consumer Methods

    // Map is Adapter Method
    let v: Vec<i32> = vec![1, 2, 3];
    // Collect is Consumer Method
    // Below '_' is type placeholder
    let collection: Vec<_> = v.iter().map(|x| x + 1).collect();

    assert_eq!(collection, vec![2, 3, 4]);
}
