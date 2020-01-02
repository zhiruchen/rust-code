mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_waiting_list();
    hosting::add_waiting_list();
    hosting::add_waiting_list();
}