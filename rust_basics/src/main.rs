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
    // let (x, y, z) = tup;
    // or (we can use index)
    // let getY = tup.1
    // Array
    // let a: [i32; 5] = [1, 2, 3, 4, 5];

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
// fn some_function(x: i32) -> i32 {
//     x + 5
// }

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
