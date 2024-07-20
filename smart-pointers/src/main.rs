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

    ref_example();

    // ----------------------------------- Reference Cycle -----------------------------------

    cycle_example();

    // ----------------------------------- Weak Pointers -----------------------------------

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>, // We are using 'Weak' here to avoid situation where children owns parent
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // To see proper value we need to turn 'Weak' into 'Rc', so we use 'upgrade' to do so
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong count = {}, leaf weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 'downgrade' turns 'Rc' into 'Weak'
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "branch strong count = {}, branch weak count = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong count = {}, leaf weak count = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong count = {}, leaf weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// ----------------------------------- Interior Mutability -----------------------------------

use crate::RefList::{OtherCons, OtherNil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum RefList {
    OtherCons(Rc<RefCell<i32>>, Rc<RefList>),
    OtherNil,
}

fn ref_example() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(OtherCons(Rc::clone(&value), Rc::new(OtherNil)));

    let b = OtherCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = OtherCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

// ----------------------------------- Reference Cycle -----------------------------------

use crate::RefCycleList::{RefCons, RefNil};

#[derive(Debug)]
enum RefCycleList {
    RefCons(i32, RefCell<Rc<RefCycleList>>),
    RefNil,
}

impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            RefCons(_, item) => Some(item),
            RefNil => None,
        }
    }
}

fn cycle_example() {
    let a = Rc::new(RefCons(5, RefCell::new(Rc::new(RefNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(RefCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
