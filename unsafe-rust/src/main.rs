fn main() {
    let mut num = 5;

    // Raw Pointers
    // '*' is not dereference but definition for raw pointer in below examples
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r3 = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r3 is: {}", *r3);
    }

    unsafe fn dangerous() {}

    // We cannot call it directly as usual:
    // dangerous();

    unsafe {
        dangerous();
    }
}
