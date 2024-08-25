use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// Attribute-like macros

fn main() {
    // Procedural macros are like functions. They take code as input, operate on that code and produce code as output.

    // There are 3 types of procedural macros:
    // * custom-derived
    // * attribute-like
    // * function-like

    Pancakes::hello_macro();
}

// THIS CRATE IS USING 'hello_macro' LIB !!!
