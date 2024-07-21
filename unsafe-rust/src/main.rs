fn main() {
    let mut num = 5;

    // Raw Pointers
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}
