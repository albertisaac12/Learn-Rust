fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist(); // needed to use the super because the use of hosting was out of the scope
    }
}
mod customer1 {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // needed to use the super because the use of hosting was out of the scope
    }
}


fn main(){
    eat_at_restaurant();
    hosting::add_to_waitlist();
}
