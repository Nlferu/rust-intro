extern crate proc_macro;

use proc_macro::TokenStream; // This allows us to read and manipulate rust code
use quote::quote; // This can take this syntax tree data structure and turn it back into rust code
use syn; // This is short from 'syntax' and it allows us to take a string of rust code and turn it into syntax tree data structure that we could operate on

// Function below is responsible for parsing the token stream into a syntax stream
#[proc_macro_derive(HelloMacro)] // This means that this is a custom derived macro with the name 'HelloMacro'
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate
    // Below is parsing our input which is token stream into an abstract syntax tree
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    // Below function is responsible for transforming syntax tree
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!(
                    "Hello, Macro! My name is {}!",
                    stringify!(#name)
                );
            }
        }
    };

    // Below is converting output into token stream by calling 'into()' method
    gen.into()
}
