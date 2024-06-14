use std::ops::Deref;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // Instead of '&T' we can do &Self::Target
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    // Value '5' is stored on heap and 'b' is pointer for that value stored on stack
    let b = Box::new(5);

    println!("b = {}", b);

    // Nil is the end of the list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("List: {:?}", list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let y = Box::new(x);
    assert_eq!(5, *y);

    let y = MyBox::new(x);
    // Below are the same as MyBox contain Deref trait
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
