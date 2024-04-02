use crate::median_and_mode::median_and_mode;

pub mod converter;
pub mod interface;
pub mod median_and_mode;

fn main() {
    // --------------------------------------------------------------------- \\
    // ------------------------------ Vectors ------------------------------ \\
    // --------------------------------------------------------------------- \\

    // Vectors are stored on HEAP

    // To create a new empty vector, we call the Vec::new function
    let _v: Vec<i32> = Vec::new();

    // We can also use macro to create vector like below
    let mut v = vec![1, 2, 3];

    // ------------------------------ Updating Vector ------------------------------ \\
    v.push(66);

    println!("Vector Data: {:?}", v);

    // ------------------------------ Reading Vector ------------------------------ \\

    // Indexing
    let third: &i32 = &v[2];
    println!("The third element is: {third}");

    // Get
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // This will panic without handling error
    // let does_not_exist = &v[100];
    // println!("Not Exist: {}", does_not_exist);

    // This will let us handle error and will return None
    let does_not_exist = v.get(100);
    println!("Does Not Exist: {:?}", does_not_exist);

    // ------------------------------ Iterating Over Vector ------------------------------ \\

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // Without `*` we would be updating borrowed value, so &v
        *i += 50;
        println!("{i}");
    }

    println!("Updated Vector: {v:?}");

    // ------------------------------ Using an Enum to Store Multiple Types ------------------------------ \\

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(666),
    ];

    println!("Enum Type Vactor: {row:?}");

    // ------------------------------ Strings ------------------------------ \\

    // Creating empty string
    let mut _s = String::new();

    // Creating string with initial value
    let data = "initial contents";
    let _s = data.to_string();
    // This method also works on a literal directly:
    let _s = "initial contents".to_string();

    // Creating string from literal value. It is the same as `to_string()`
    let s = String::from("initial contents");

    // ------------------------------ Modifying Strings ------------------------------ \\

    // Using `format!()` macro
    let modified = format!("{} and {}", s, "additional");

    println!("Modified String: {}", modified);

    // Using `push_str()`
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    println!("Pushed String: {}", s);

    // We can also use `push()` to add one character at the end of string
    let mut s = String::from("lo");
    s.push('l');

    // Adding two strings
    // We are using s1 and &s2 here as adding strings is using following fn `fn add(self, s: &str) -> String {}`
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    println!("Sum Of Strings: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;
    // Same with format
    // Redefining s1 as we lost it in above _s
    let s1 = String::from("tic");
    // Format macro does not take ownership of any of parameters
    let s = format!("{s1}-{s2}-{s3}");

    println!("Sum Of Strings: {}", s);

    let hell = String::from("Hola");
    let hello = String::from("Здравствуйте");

    let leng_one = hell.len();
    let leng_two = hello.len();

    println!(
        "Unicode Scalar takes 2 bytes instead of 1 as usual string: {} and {}",
        leng_one, leng_two
    );

    let _hello = "Здравствуйте";
    // Indexing string in Rust is not allowed as unicode chars would throw unafmiliar numbers
    // let answer = &hello[0];

    // ------------------------------ Slicing Strings ------------------------------ \\
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("Sliced String: {}", s);

    let hello = "rusti";
    let s = &hello[0..4];

    println!("Sliced String: {}", s);

    // ------------------------------ Iterating Strings ------------------------------ \\

    // For individual Unicode scalar values
    for c in "Зд".chars() {
        println!("{c}");
    }

    // For bytes method
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // ------------------------------ Hash Map ------------------------------ \\

    // Hash Maps Data is stored on HEAP

    // Creating new hash map
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // Adding data to Hash Map
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // ------------------------------ Accessing Values in Hash Maps ------------------------------ \\

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Blue Team Score: {}", score);

    // ------------------------------ Iterating Hash Maps ------------------------------ \\

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ------------------------------ Updating Hash Maps ------------------------------ \\

    // Overwriting Value
    scores.insert(String::from("Blue"), 17);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present

    // Checking if key exist
    let maka = scores.entry(String::from("Yellow"));
    println!("Does Key Yellow exist: {:?}", maka);

    // Checking and adding data if key is free
    scores.entry(String::from("Yellow")).or_insert(66);
    scores.entry(String::from("Red")).or_insert(66);

    let modfified_data = scores.get(&String::from("Yellow")).copied().unwrap_or(0);
    let modfified_data_red = scores.get(&String::from("Red")).copied().unwrap_or(0);

    println!("Value for Yellow key: {}", modfified_data);
    println!("Value for Red key: {}", modfified_data_red);
    println!("{scores:?}");

    // Updating a Value Based on the Old Value

    // Program below is counting how many times each word is present in whole text, if there is new word we will add it to map
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // Without `*` we would be updating borrowed value, so &v
        *count += 1;
    }

    println!("{:?}", map);

    // Median And Mode Exercise

    let even_data = vec![3, -5, 88, 666, 666, -777, -33, 777, 666, 0, 88, -11];
    let odd_data = vec![3, -5, 88, 666, 666, -777, -33, 777, 666, 0, 88, -11, 99];

    let (median, mode) = median_and_mode(even_data);
    println!("Median Value Is: {:?} Mode Value Is: {}", median, mode);
    let (median, mode) = median_and_mode(odd_data);
    println!("Median Value Is: {:?} Mode Value Is: {}", median, mode);

    //
}
