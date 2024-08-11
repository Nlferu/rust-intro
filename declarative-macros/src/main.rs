fn main() {
    let v1: Vec<u32> = vec![1, 2, 3];
    let v2: Vec<&str> = vec!["a", "b", "c", "d", "e"];

    println!("Vector: {:?}", v1);
    println!("Vector: {:?}", v2);
}
