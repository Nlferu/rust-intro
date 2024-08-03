use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[allow(dead_code)]
struct Age(u32);

#[allow(dead_code)]
struct ID(u32);

fn main() {
    // =============================
    //        Newtype Pattern
    // =============================

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);

    println!("w = {}", w);

    // ==========================
    //        Type Aliases
    // ==========================

    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f: Thunk = Box::new(|| println!("hi"));

    #[allow(dead_code)]
    fn takes_long_type(_f: Thunk) {
        // --snip--
    }

    #[allow(dead_code)]
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
}
