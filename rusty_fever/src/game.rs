use crate::menu::{Menu, level_1_menu};
use crate::menu_item::MenuItem;
use crate::customer::Customer;


pub fn play() {
    // Get the default menu
    let menu: Menu = level_1_menu();
    // Create 20 customers -> queue
    let mut customers: Vec<Customer> = Vec::new();
    for i in (1..21).enumerate() {
        customers.push(Customer::new(i, 5));
    }
}