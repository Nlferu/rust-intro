use smart_pointers;
use std::ops::Deref;
use List::{Cons, Nil};

// Rc<T> enables multiple owners of the same data;
// Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time;
// Rc<T> allows only immutable borrows checked at compile time;
// RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

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

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
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

    // ----------------------------------- Deref Trait -----------------------------------

    let y = MyBox::new(x);
    // Below are the same as MyBox contain Deref trait
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("Rust"));
    // Even if this function takes &str it works because:
    // &MyBox<String> ('MyBox' has Deref trait) -> &String ('String' also has Deref trait) -> &str
    hello(&m);

    // Rust doing above type match automatically, if it would not do this we would need to call this fn like below:
    hello(&(*m)[..]);

    // Rust does deref coercion for below:
    // From '&T' to '&U' when 'T': Deref<Target=U>
    // From '&mut T' to '&mut U' when 'T': DerefMut<Target=U>
    // From '&mut T' to '&U' when 'T': Deref<Target=U>

    // Rust cannot perform deref coercion always for below:
    // From '&T' to '&mut U'

    // ----------------------------------- Drop Trait -----------------------------------

    // Drop is performed in reversed order, so first we drop 'd' then 'c' -> Look prints log order after 'cargo run'
    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("Other Stuff"),
    };

    println!("CustomSmartPointers created!");
    println!("D: {:?}", d);

    // We cannot do: c.drop()
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");

    // ----------------------------------- Reference Counting -----------------------------------

    smart_pointers::sep_main();

    // ----------------------------------- Interior Mutability -----------------------------------

    let a = 5;
    let b = &mut a;

    let mut c = 10;
    let d = &c;
    *d = 20;
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
