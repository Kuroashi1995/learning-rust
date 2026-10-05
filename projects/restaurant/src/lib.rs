mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //Absolute pathing
    front_of_house::hosting::add_to_waitlist();

    //Relative pathing
    front_of_house::hosting::add_to_waitlist();
}
