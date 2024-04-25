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

    // Now lifetime of 'string1' is longer than lifetime of 'string2'
    {
        let string2 = String::from("xyz");

        // We tell to borrow checker that whatever gets returned from longest() will have a lifetime that is equal to the smallest lifetime being passed in
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }
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
