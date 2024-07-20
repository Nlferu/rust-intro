#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method -> associated functions that are associated with our custom type which is Rectangle in this case
impl Rectangle {
    // We dont want to take ownership, so we use `&`
    // And we just want to read data not write to it
    fn area_method(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

// We can split this `impl` block too if we want
impl Rectangle {
    // Self == Rectangle -> so it is simply matching type
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
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

    // Calling Method from Rectangle
    println!(
        "Structs Method: Area of recktangle is {} square pixels -> is width > 0? {}",
        // Rust automatically adds &, so below is equal to (&rectangle).area_method()
        rectangle.area_method(),
        rectangle.width()
    );

    // Check whether the width and height of self are greater than the width and height of the other Rectangle
    let rectangle_two: Rectangle = Rectangle {
        width: 22,
        height: 38,
    };

    println!(
        "Check Can Hold: {} and {}",
        // We can call it like this as can_hold() has '&self' argument:
        rectangle.can_hold(&rectangle_two),
        // We can also call it like this:
        Rectangle::can_hold(&rectangle, &rectangle_two)
    );

    // Square -> it creates square instead of rectangle

    let sq: Rectangle = Rectangle::square(3);
    // If square() doesnt have '&self' we cannot call it as below:
    //let sq: Rectangle = rectangle.square(3);

    println!("Square: {:?}", sq);
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
