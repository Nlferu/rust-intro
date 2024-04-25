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
// To fix it we need to specify lifetime by adding <'a> -> a here is optional as we can use whatever we what for our lifetime name
// Adding {&'a} -> we just say that x will use our lifetime and y will use our lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
