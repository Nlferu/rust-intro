#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Basic
    let width: u32 = 30;
    let height: u32 = 50;

    println!(
        "Basic: Area of recktangle is {} square pixels",
        area(width, height)
    );

    // Tuple
    let rectangle: (u32, u32) = (30, 50);

    println!(
        "Tuple: Area of recktangle is {} square pixels",
        area_tuple(rectangle)
    );

    // Structs
    let rectangle: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    // Printing Whole Rectangle Struct
    println!("Rectangle Struct: {:?}", rectangle);

    // We are using reference here to keep `main()` as owner of this struct
    println!(
        "Structs: Area of recktangle is {} square pixels",
        area_structs(&rectangle)
    );

    // Printing with `dbg!` macro
    dbg!(&rectangle);
}

// Basic version
fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

// Tuples version
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    // We dont need to use return as it is built in
    dimensions.0 * dimensions.1
}

// Structs version
fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
