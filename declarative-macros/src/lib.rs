// Simplified version of macro from std:
// Below means that whenever the crate in which this macro is defined in is brought into scope this macro should be made available
#[macro_export]
macro_rules! vec {
    // vec![1, 2, 3];
    // First we match 1, which is expression and assign it to $x, then we match 2 and assign same way and so on...
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // Below means: Generate this line of code for every match that we get
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
