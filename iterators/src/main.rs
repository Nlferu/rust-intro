fn main() {
    let v = vec![1, 2, 3];

    let v_iter = v.iter();

    for value in v_iter {
        println!("Current Value: {}", value);
    }
}
