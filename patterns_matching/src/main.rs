fn main() {
    // =====================================
    //             Match Expressions
    // =====================================

    #[derive(Debug)]
    enum Language {
        English,
        Japanese,
        French,
        Spanish,
        Chinese,
    }

    let language = Language::English;

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
    }
}
