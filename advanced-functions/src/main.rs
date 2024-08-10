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

    // Function pointers

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let alt_list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("{:?}", list_of_strings);
    println!("{:?}", alt_list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        _Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("Statuses: {:?}", list_of_statuses);

    // Returning closures

    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let closure = returns_closure();

    println!("Closure: {}", closure(10));
}
