use core::slice;

fn main() {
    // =======================================
    //        Dereference A Raw Pointer
    // =======================================

    let mut num = 5;

    // Raw pointers
    // '*' is not dereference but definition for raw pointer in below examples
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let _r3 = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // Below gives us: Segmentation fault
        // println!("r3 is: {}", *r3);
    }

    // ===========================================
    //      Call An Unsafe Function Or Method
    // ===========================================

    unsafe fn dangerous() {}

    // We cannot call it directly as usual:
    // dangerous();

    unsafe {
        dangerous();
    }

    // Safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Interaction with external code in different language
    unsafe { println!("Absolute value of -3 according to C: {}", abs(-3)) }

    // ==================================================
    //     Access Or Modify A Mutable Static Variable
    // ==================================================

    // Changing value of static variable:
    static HELLO_WORLD: &str = "Hello World";
    println!("Name is: {}", HELLO_WORLD);

    // 'static' has fixed address in memory meaning it always accessing the same data. 'static' can be mutable
    // 'const' are allowed to duplicate their data whenever they're used

    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    // Not allowed!
    // println!("COUNTER: {}", COUNTER);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // =======================================
    //        Implement An Unsafe Trait
    // =======================================

    unsafe trait Foo {
        // Method go here
    }

    unsafe impl Foo for i32 {
        // Method implementation go here
    }

    // =====================================
    //        Access Fields Of Unions
    // =====================================
}

// Interaction with external code in different language
extern "C" {
    fn abs(input: i32) -> i32;
}

// Allowing other languages to call our Rust functions:
// The #[no_mangle] attribute can be used on items to disable name mangling on that item. We prevent name change of function.
// The #[export_name] attribute can be used to specify the exact name that will be used for a function or static.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Unsafe implementation of below:
// Safe abstraction of unsafe code
#[allow(dead_code)]
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Safe implementation
// It is good to use, but Rust is not smart enough to figure it out
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }
