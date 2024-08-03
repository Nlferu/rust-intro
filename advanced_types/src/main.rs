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

    // ========================
    //        Never Type
    // ========================

    // It means that this function will never return
    #[allow(dead_code)]
    fn bar() -> ! {
        panic!("This function will never return!");
    }

    let game_in_progress: bool = false;

    while game_in_progress {
        let guess = "5";

        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // continue has never type
            Err(_) => continue,
        };
    }

    // panic! also returns never type

    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }
}
