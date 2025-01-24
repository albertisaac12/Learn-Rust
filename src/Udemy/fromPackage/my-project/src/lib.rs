
// // mod customer_experience{
// // mod front_of_house {
// //     mod hosting {
// //         fn add_to_waitlist() {}
// //     }
// // }

// // pub fn eat_at_restaurant() {
// //     // Absolute path
// //     crate::customer_experience::front_of_house::hosting::add_to_waitlist();
// //     // for absoulte path we always start with the create 

// //     // Relative path
// //     front_of_house::hosting::add_to_waitlist();
// // }
// // }

// // super refers to the parent module in rust 

// // mod front_of_house {
// //     mod hosting {
// //         fn add_to_waitlist() {}
// //     }
// // }

// // pub fn eat_at_restaurant() {
// //     // Absolute path
// //     crate::front_of_house::hosting::add_to_waitlist();
// //     // for absoulte path we always start with the create 

// //     // Relative path
// //     front_of_house::hosting::add_to_waitlist();
// // }

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// mod dining {
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//     // for absoulte path we always start with the create 

//     // Relative path
//     front_of_house::hosting::add_to_waitlist(); // still valid in this case because both have the create as the parent module
// }

// }

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();

//     // let c = if 1 == 1 {2}else {"fuck"};
// }


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

/*
    In the relative path, the logic is the same as the absolute path except for the first step: 
    rather than starting from the crate root, the path starts from front_of_house. 
    The front_of_house module is defined within the same module as eat_at_restaurant, 
    so the relative path starting from the module in which eat_at_restaurant is defined works. 
    Then, because hosting and add_to_waitlist are marked with pub, the rest of the path works, and this function call is valid!

*/

