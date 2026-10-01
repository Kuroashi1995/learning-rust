mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn sit_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //Absolute pathing
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative pathing
    front_of_house::hosting::add_to_waitlist();
}
