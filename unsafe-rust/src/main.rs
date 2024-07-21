use core::slice;

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

    // Safe abstraction over unsafe code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// Unsafe Implementation of below:
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

// Safe Implementation
// It is good to use, but Rust is not smart enough to figure it out
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }
