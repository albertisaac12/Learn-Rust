mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // for absoulte path we always start with the create 

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}