fn add_one(x: i32) -> i32 {
    x + 1
}

// Below 'fn' is type and not trait here
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Alternative version of above function:
// Below 'Fn' is closure trait bound
fn do_twice_gen<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

// Function Traits:
// Fn - (closure captures variables immutably)
// FnMut - (closure captures variables mutably)
// FnOnce - (closure takes onwership of the values in it's environment thus consuming variables)

fn main() {
    let answer = do_twice(add_one, 6);
    let _same_answer = do_twice_gen(add_one, 6);

    println!("The answer is: {}", answer);
}
