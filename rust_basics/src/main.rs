fn main() {
    // Hello World
    let hi_msg: &str = "Hello Rust World!";
    println!("{hi_msg}");

    // Scalar Types:
    // Length	Signed	Unsigned    Max ( i / u )
    // 8-bit	i8	    u8          - 128 to 127 / 255
    // 16-bit	i16	    u16         / 65535
    // 32-bit	i32	    u32         / 4294967295
    // 64-bit	i64	    u64         / 18446744073709551615
    // 128-bit	i128	u128        / 340282366920938463463374607431768211455
    // arch	    isize	usize       Depends on system we use windows 64 bits for example
    // Signed numbers range: -(2^(n - 1)) to 2^(n - 1) - 1 -> where `n` is bits we used

    // Other Types:
    // Number literals	Example
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // We can add _ sign on start to ignore unused warning for variables and functions
    let _unused_variable: i32;

    // Unsigned integers can be only positive
    let unsigned: u8 = 255;
    // Signed integer can be negative
    let signed: i8 = -128;
    // Float
    // Floats are signed and only f32 or f64
    // let x = 2.0; // f64 type
    // let x: f32 = 3.0; // f32 type
    let float: f32 = 1.215;
    // String
    let letter: &str = "some string";
    // Emoji
    let emoji: &str = "\u{1F600}";
    // Boolean
    let is_true: bool = true;
    // Reminder
    let remaining: u8 = 43 % 5;
    // Division
    // Results in -1
    let truncated: i8 = -5 / 3;
    // Power
    let base: u16 = 2;
    let powered: u16 = base.pow(10);
    // Char -> always use '' instead of ""
    let z: char = 'ℤ';
    // Tuple -> those are fixed size (immutable array) can have multi types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // We can get each value from tuple as below:
    let (_x, _y, _z) = tup;
    // or (we can use index)
    let _get_y = tup.1;
    // Array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    println!(
        "unsigned: {unsigned}, signed: {}, float: {}, letter: {}, emoji: {}, boolean: {}, mod: {}, truncated: {truncated}, power: {powered}, char: {z}, tuple: {:?}",
        signed, float, letter, emoji, is_true, remaining, tup
    );

    // Mutability
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 7;
    println!("The value of x is: {x}");

    // Constants
    // We ar not allowed to use `mut` with const items
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Const Value: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // We can perform a few transformations on a value but have the variable be immutable after those transformations have been completed
    let y: i8 = 5;

    let y: i8 = y + 1;

    {
        let y: i8 = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of outer y is: {y}");

    // The other difference between `mut` and `shadowing` is that we can override type with `shadowing` and we cannot do it with `mut` (within `mut` type have to match)
    let spaces: &str = "      ";
    let spaces: usize = spaces.len();
    println!("Spaces: {spaces}");

    // Arrays:
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!(
        "Index: {}, Length: {}, Array: {:?}",
        arr[0],
        other_arr.len(),
        arr
    );

    println!(
        "Index: {}, Length: {}, Array: {:?}",
        arr[0],
        other_arr.len(),
        other_arr
    );

    println!("some_fn: {:?}", some_fn(1));

    loop_fn();
    counter();
    while_lo();
    for_loop();

    // Vectors
    let words: Vec<&str> = vec!["hello", "world"];
    println!("Words that are often combined: {:?}", words);

    // Debug
    dbg!("hello world".split(' ').collect::<Vec<_>>()).join("-");

    // Stack memory example (size of variable known):
    let _l: i32 = 10;
    // Stack memory types examples:

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // Heap memory example (size of variable unknown):
    let _k: String = String::from("hello");

    // Below does not apply to Stack memory...
    // Heap clean memory example:
    let s1 = String::from("hello");
    // If we just type: let s2 = s1; -> s1 will become invalid (to avoid double same memory clean)
    // So if we still want to clone it we can use below function
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn some_fn(x: i32) -> i32 {
    if x > 1 && x < 100 {
        return x;
    } else if x == 100 {
        return x + 8;
    }

    return x + 5;
}

// We can do the same as above like this as Rust do not require `return` keyword:
fn _some_function(x: i32) -> i32 {
    x + 5
}

fn loop_fn() {
    let mut counter: u8 = 0;

    let result: u8 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    return println!("The result is: {result}");
}

fn counter() {
    let mut count: i32 = 0;

    // Loop below has label -> `counting_up`, so we can tell break to do something with this loop
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count: {count}");
}

fn while_lo() {
    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a: [u8; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("The reversed loop in range 1-4 is: {number}");
    }
}

fn _to_do() {
    let smth: &str = "Something to do";

    // It will always panic as todo = panic()
    todo!("Write a function to reverse {smth}");
}

// Below shows an example of ownership of variables for stack and heap memory
// --------------------------------------------------------------------------------------------------------------- \\
fn _stack_and_heap() {
    let s: String = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // println!("{}, s"); -> ... and so is no longer valid here and we cannot print it

    let x: i32 = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
    println!("{x}"); // but i32 is Copy, so it's okay to still
                     // use x afterward
}

pub fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

pub fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

fn _stack_and_heap_two() {
    let s1: String = gives_ownership(); // gives_ownership moves its return
                                        // value into s1

    let s2: String = String::from("hello"); // s2 comes into scope
    println!("{}", s2);

    let s3: String = takes_and_gives_back(s2); // s2 is moved into
    println!("{}", s1);
    // println!("{}", s2); -> s2 is no longer valid
    println!("{}", s3); // takes_and_gives_back, which also
                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

pub fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it

    let some_string: String = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
pub fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

// --------------------------------------------------------------------------------------------------------------- \\

// If we want to avoid losing ownership of our variable we can use references `&` dereference `*`
fn _stack_and_heap_three() {
    let s1: String = String::from("hello");

    let len: usize = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

pub fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to
// To modify we have to edit `&` into `&mut`
fn _stack_and_heap_four() {
    let mut s: String = String::from("hello");

    change(&mut s);
}

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
fn _mutme() {
    let mut s: String = String::from("hello");

    let r1: &mut String = &mut s;

    // cannot borrow `s` as mutable more than once at a time \\
    // let r2: &mut String = &mut s; -> THIS WILL FAIL

    println!("{}", r1);

    let r1: &String = &s; // no problem
    let r2: &String = &s; // no problem

    // cannot borrow `s` as mutable because it is also borrowed as immutable \\
    // let r3 = &mut s; -> BIG PROBLEM

    println!("{}, {}", r1, r2);

    let r1: &String = &s; // no problem
    let r2: &String = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3: &mut String = &mut s; // no problem
    println!("{}", r3);
}
