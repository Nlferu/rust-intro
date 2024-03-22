fn main() {
    // Basic
    let width: u32 = 30;
    let height: u32 = 50;

    println!(
        "The area of recktangle is {} square pixels",
        area(width, height)
    );

    // Tuple
    let rectangle: (u32, u32) = (30, 50);

    println!(
        "The area_tuple of recktangle is {} square pixels",
        area_tuple(rectangle)
    );
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
