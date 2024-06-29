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

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // ======================================
    //             Let Statements
    // ======================================
}
