mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}

        fn _serve_order() {}

        fn _take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // `crate` means root

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
