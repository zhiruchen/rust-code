mod front_of_house;

fn main() {
    println!("Hello, world!");
    proj_manage::eat_at_restaurant();
    
    println!("call hosting::add_waiting_list from main");
    front_of_house::hosting::add_waiting_list();
}
