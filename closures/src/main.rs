use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn _simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);

    // Testing refactored Cacher
    let mut cacher = Cacher::new(|x| x + 1);
    println!("{}", cacher.value(1)); // 2
    println!("{}", cacher.value(2)); // 3
    println!("{}", cacher.value(1)); // 2, cached result

    // Anonymous vs Normal fn
    let x = 4;

    let equal_to = |z| z == x;
    // We cannot do below because normal fn can't capture dynamic environment in a fn item
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }

    let y = 4;

    assert!(equal_to(y));

    // Capturing environtment is encoded in functions traits: FnOnce, FnMut, Fn
    // FnOnce -> closures can't take ownership of the same variables more than once (these closures can only be called once)
    // FnMut -> mutably borrows values
    // Fn -> immutably borrows values

    let x = vec![1, 2, 3];

    // In case we would like our 'equal_to' anon function to take ownership of 'x' we do below:
    // let equal_to = move |z| z == x;
    let equal_to = |z| z == x;

    println!("Can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to(y));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // Using HashMap instead of below:
    // value: Option<u32>,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            // Using HashMap instead of below:
            // value: None,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                // Using HashMap instead of below:
                // self.value = Some(v);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Example closures
    let _example_closure_one = |x: u32| x;
    let _example_closure = |x: u32| -> u32 { x };

    // Anonymous function with 'num' parameter
    let mut cached_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cached_result.value(intensity))
        }
    }
}
