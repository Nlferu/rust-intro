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
    let s1: String = String::from("hello");
    // If we just type: let s2 = s1; -> s1 will become invalid (to avoid double same memory clean)
    // So if we still want to clone it we can use below function
    let s2: String = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    slices();

    let slice_string: String = String::from("hell word is a demon");
    let slice_string_two: &str = "hell word is a demon";
    println!("{}", first_word(&slice_string));
    // println!("{}", first_word(slice_string_two)); -> ERROR
    // println!("{}", first_word(&slice_string_two)); -> ERROR
    // As we can see everything works for corrected fn
    println!("{}", first_word_corrected(&slice_string));
    println!("{}", first_word_corrected(slice_string_two));
    println!("{}", first_word_corrected(&slice_string_two));

    // --------------------------------------------------------------------- \\
    // ------------------------------ Structs ------------------------------ \\
    // --------------------------------------------------------------------- \\
    // Example (Order of data in struct doesnt matter)
    struct User {
        active: bool,
        username: String,
        _email: String,
        _sign_in_count: u64,
    }

    let mut user_one: User = User {
        active: true,
        username: String::from("Hastur"),
        _email: String::from("somemail@example.com"),
        _sign_in_count: 1,
    };

    println!("User One Username: {}", user_one.username);

    user_one.active = false;

    fn _build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // shorthand instead of `username: username`
            _email: email,
            _sign_in_count: 1,
        }
    }

    // We can also create new user using data from existing user
    let _user_two: User = User {
        active: user_one.active,
        username: user_one.username,
        _email: String::from("another@example.com"),
        _sign_in_count: user_one._sign_in_count,
    };

    // We can also fill data as below
    let _user_three: User = User {
        _email: String::from("another@example.com"),
        .._user_two // We cannot use `user_one` here as it is `mut`
    };

    // Tuple Structs
    #[derive(Debug)] // Deriving the Debug trait for Color to be able to print it
    struct Color(i32, i32, String);
    struct Point(i32, i32, i32);

    let _black: Color = Color(0, 0, String::from("another@example.com"));
    let _origin: Point = Point(0, 0, 0);
    println!("{:?}", _black);
    // Calling exact tuple element
    println!("{}", _black.2);

    // Unit-Like Struct
    struct AlwaysEqual;

    let _subject: AlwaysEqual = AlwaysEqual;

    // ------------------------------------------------------------------- \\
    // ------------------------------ Enums ------------------------------ \\
    // ------------------------------------------------------------------- \\
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four: IpAddrKind = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {
        println!("IP: {:?}", ip_kind);
    }

    route(IpAddrKind::V4);

    struct IpAddr {
        _kind: IpAddrKind,
        _address: String,
    }

    let _home: IpAddr = IpAddr {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    let _loopback: IpAddr = IpAddr {
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };

    #[derive(Debug)]
    enum IpAddress {
        V4(String),
        V6(String),
    }

    let home: IpAddress = IpAddress::V4(String::from("127.0.0.1"));
    let _loopback: IpAddress = IpAddress::V6(String::from("::1"));

    println!("Home IP Address: {:?}", home);

    enum IpAddrExample {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home: IpAddrExample = IpAddrExample::V4(127, 0, 0, 1);

    let _loopback: IpAddrExample = IpAddrExample::V6(String::from("::1"));

    struct _Ipv4Addr {
        // --snip--
    }

    struct _Ipv6Addr {
        // --snip--
    }

    enum _IpAddressStruct {
        V4(_Ipv4Addr),
        V6(_Ipv6Addr),
    }

    #[derive(Debug)]
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("Message: {:?}", self);
        }
    }

    let m: Message = Message::Write(String::from("hello"));
    m.call();

    // Below is buit in Rust, so we do not have to code it here
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // Below would use our Option enum from above
    let _some_number: Option<i32> = Option::Some(5);
    // Below is using Option enum from standard library included in prelude
    let _some_char: std::prelude::v1::Option<char> = Some('e');
    let _absent_number: Option<i32> = Option::None;

    // --------------------------------------------------------------------------- \\
    // ------------------------------ Match Pattern ------------------------------ \\
    // --------------------------------------------------------------------------- \\
    #[derive(Debug)]
    enum UsState {
        _Alabama,
        Alaska,
    }

    enum Coin {
        _Penny,
        _Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::_Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::_Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

    println!("Value in cents: {}", value_in_cents(Coin::Dime));
    println!(
        "Value in cents with state: {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let _six = plus_one(five);
    let none = plus_one(None);

    println!("None Value: {:?}", none);

    // Match have to be EXHAUSTIVE!
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Below pattern ensures we do following for any other inputs
        // We are also assigning value to `other`, so we can use it later
        other => move_player(other),
        // If we do not want to use value from other outcomes we can do below
        // --------------------------------------
        // _ => println!("All other outcomes"),
        // --------------------------------------
        // We can also do below to do nothing
        // _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {
        println!("Player lost fancy hat")
    }
    fn move_player(_num_spaces: u8) {
        println!("Player moved by {} fields", _num_spaces)
    }

    // If Let
    // We need to use _ => (), here
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Instead of above we can just do below:
    // If Let is best to use when value matches one pattern and we are ignoring any other values
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Those 2 below are the same
    let mut count = 0;
    let coin: Coin = Coin::Quarter(UsState::Alaska);
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => {
            count += 1;
            println!("Counter from match: {}", count);
        }
    }

    let coin: Coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("Counter from if let: {}", count);
    }

    // Rust Exercises
    {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        // `name` is moved out of person, but `age` is referenced
        let Person { ref name, ref age } = person;

        println!("The person's age is {}", age);
        println!("The person's name is {}", name);

        // Error! borrow of partially moved value: `person` partial move occurs
        //println!("The person struct is {:?}", person);

        // `person` cannot be used but `person.age` can be used as it is not moved
        println!("The person's age from person struct is {}", person.age);

        // We need to add ref to name here: let Person {ref name, ref age} = person; to get it working
        // So now we are able to use it endlessly
        println!("Person name: {}", person.name);
    }
}

// ---------------------------------------------------------------------------------- \\
// ------------------------------ Additional Functions ------------------------------ \\
// ---------------------------------------------------------------------------------- \\

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

    // Sumup
    // At any given time, you can have either one mutable reference or any number of immutable references
    // References must always be valid
}

// --------------------------------------------------------------------------------------------------------------- \\

// The Slice Type
fn first_word(s: &String) -> usize {
    // Converting String into bytes to check if value is space or not
    let bytes: &[u8] = s.as_bytes();

    // In each iteration, `i` represents the index of the element in the collection, and `item` represents the element itself
    // bytes.iter(): This returns an iterator over the elements of the bytes collection
    // enumerate(): This wraps the iterator returned by iter() and pairs each element with its index.
    // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
    for (i, &item) in bytes.iter().enumerate() {
        // Searching for spaces -> if found return it's position (index)
        // Otherwise we return the length of the string by s.len()
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// `first_word()` function corrected with slice type
// Using `s: &str` instead of `s: &String` is much more flexible and allows more inputs
fn first_word_corrected(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// String Slices
fn slices() {
    let s: String = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("{} {}", hello, world);

    // Below are equal
    let _slice: &str = &s[0..2];
    // ==
    let _slice: &str = &s[..2];

    // Below are equal
    let len: usize = s.len();
    let _slice: &str = &s[3..len];
    // ==
    let _slice: &str = &s[3..];

    // Below are equal (Entire String)
    let _slice: &str = &s[0..len];
    // ==
    let _slice: &str = &s[..];
}
