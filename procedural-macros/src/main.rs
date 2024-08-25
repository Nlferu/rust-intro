use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // Procedural macros are like functions. They take code as input, operate on that code and produce code as output.

    // There are 3 types of procedural macros:
    // * custom-derived
    // * attribute-like
    // * function-like

    Pancakes::hello_macro();

    // Attribute-like macros

    // #[route(GET, "/")]
    // fn index() {
    //     // ...
    // }

    // #[proc_macro_attribute]
    // pub fn route(
    //     attr: TokenStream, // GET, "/"
    //     item: TokenStream, // fn index() {}
    // ) -> TokenStream {
    //     // ...
    // }

    // Function-like macros

    // let sql = sql!(SELECT * FROM posts WHERE id=1);

    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {
    //     // ...
    // }
}

// THIS CRATE IS USING 'hello_macro' LIB !!!
