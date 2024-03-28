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
}
