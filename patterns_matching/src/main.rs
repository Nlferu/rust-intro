fn main() {
    // =====================================
    //             Match Expressions
    // =====================================

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Language {
        English,
        Japanese,
        French,
        Spanish,
        Chinese,
    }

    let language = Language::Japanese;

    match language {
        Language::English => println!("Hello World!"),
        Language::Japanese => println!("こんにちは世界!"),
        Language::French => println!("Bonjour le monde!"),
        Language::Spanish => println!("Hola Mundo!"),
        lang => println!("Unsupported language! {:?}", lang),
        // or
        // _ => println!("Unsupported language!"),
    }

    // =============================================
    //         Conditional if let Expression
    // =============================================

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privilaged")
        } else {
            println!("Authorization status: basic")
        }
    } else {
        println!("Authorization status: guest")
    }

    // =============================================
    //         Conditional Loops while let
    // =============================================

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // =====================================
    //               For Loops
    // =====================================

    let v = vec!['a', 'b', 'c'];

    // Enumerate returns 'tuple' with 'index' and 'value'
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // ======================================
    //             Let Statements
    // ======================================

    let _x = 5;

    // let PATTERN = EXPRESSION;

    let (_x, _y, _) = (1, 2, 3);

    // ===========================================
    //             Function Parameters
    // ===========================================

    let points = (3, 5);
    print_coordinates(&points);

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current Location: ({}, {})", x, y);
    }

    // ==================================================
    //           Irrefutable Refutable Patterns
    // ==================================================

    // Irrefutable (Pattern that will always match)
    let _x = 5;

    // Refutable (Pattern that will NOT always match)
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    };

    // Can only accept irrefutable patterns:
    // function parameters
    // let statements
    // for loops

    // Errors Examples

    let _x: Option<&str> = None;
    // Refutable pattern in local binding
    // let Some(x) = x;

    // Irrefutable `if let` pattern
    #[allow(irrefutable_let_patterns)]
    if let x = 5 {
        println!("{}", x);
    }

    // ================================================
    //                  Pattern Syntax
    // ================================================

    let x = Some(5);
    let _y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Simpler approach
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}", x)
        }
        Point { x: 0, y } => {
            println!("On the y axis at {}", y)
        }
        Point { x, y } => {
            println!("On neither axis: ({}, {})", x, y)
        }
    }

    //====================
    // Destructuring Enums
    //====================

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit")
        }
        Message::Move { x, y } => {
            println!("Move to x: {} y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color: red {}, green {}, blue {}", r, g, b);
        }
    }

    //===========================
    // Destructuring Nested Enums
    //===========================

    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Msg {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Msg::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Msg::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color: red {}, green {}, blue {}", r, g, b);
        }
        Msg::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color: hue {}, saturation {}, value {}", h, s, v);
        }
        _ => (),
    }

    //===============================
    // More Complicated Destructuring
    //===============================

    #[allow(dead_code)]
    struct Points {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("Feet: {}, Inches: {}", feet, inches);
    println!("X: {}, Y: {}", x, y);

    //============================
    // Ignoring Values In Patterns
    //============================

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    let s = Some(String::from("value"));

    // Try putting in '_s'
    if let Some(_) = s {
        println!("Found a string");
    }

    println!("{:?}", s);

    #[allow(dead_code)]
    struct Po {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Po { x: 0, y: 0, z: 0 };

    match origin {
        Po { x, .. } => println!("x is {} and we do not care for rest of the values..", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    //=============
    // Match Guards
    //=============

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Any condition: {}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("No"),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
