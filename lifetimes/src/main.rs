fn main() {
    let x = 5;

    let r = &x;

    println!("r: {r}");

    let string1 = String::from("absc");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);
}

// This function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
