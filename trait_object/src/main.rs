use trait_object::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Draw SelectBox
    }
}

fn main() {
    println!("Hello, world!");
}
