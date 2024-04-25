fn main() {
    // fn main() {
    //     let r;                // ---------+-- 'a -> lifetime of 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  | -> lifetime of 'b
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    let x = 5;

    let r = &x;

    println!("r: {r}");

    let string1 = String::from("absc");
    let result: &str;
    // Now lifetime of 'string1' is longer than lifetime of 'string2'
    {
        let string2 = String::from("xyz");

        // We tell to borrow checker that whatever gets returned from longest() will have a lifetime that is equal to the smallest lifetime being passed in
        result = longest(string1.as_str(), string2.as_str());
        // If we try to print below out of this scope we will get error saying that lifetime of borrowed 'string2' value does not live long enough,
        // because it just ends in this scope
        println!("The longest string is: {}", result);
    }

    #[allow(dead_code)]
    struct ImportExcerpt<'a> {
        part: &'a str,
    }

    // Look at RULES to see why we do not need to specify 'a in below
    impl<'a> ImportExcerpt<'a> {
        fn _return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }

        // With 'a it would liike like (actually looks)
        fn __return_part(&'a self, announcement: &str) -> &'a str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Devil. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let _i = ImportExcerpt {
        part: first_sentence,
    };

    ///////////////////////////
    //    STATIC LIFETIME    //
    ///////////////////////////

    // This is special lifetime means that reference can live as long as the duration of the program
}

// This function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y
// To fix it we need to specify lifetime by adding <'a> -> a here is optional as we can use whatever we what for our lifetime name
// Adding {&'a} -> we just say that x will use our lifetime and y will use our lifetime
// Once we added 'a to all variables and return type we just explain relationship between them. Smallest lifetime will be always picked
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetimes does not change lifetime of variable but explains relationship between variables
// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime

/////////////////
//    RULES    //
/////////////////
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// Rule 3 only applies for methods
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters.

// In this case compiler automatically added 'a lifetimes to our function below, so we do not need to specify it
fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// But actually it looks like see below -> ...
fn _first_word_a<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
