mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// We have two options of pathing here new and old
// src/front_of_house.rs (what we covered)
// src/front_of_house/mod.rs (older style, still supported path)

// src/front_of_house/hosting.rs (what we covered)
// src/front_of_house/hosting/mod.rs (older style, still supported path)
