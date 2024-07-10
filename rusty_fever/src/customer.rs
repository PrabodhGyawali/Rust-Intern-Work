use rand::*;
use crate::{menu::Menu, menu_item::MenuItem};
pub struct Customer {
    pub id: u32,
    pub is_waiting: bool,
    pub wait_time: u8,
    pub nice: u8,   // for tipping algorithm and wait-time
    pub order: Option<MenuItem>,
}

impl Customer {
    pub fn new(id: u32, nice: u8) -> Self {
        Customer {
            id: id, 
            is_waiting: true, 
            wait_time: calculate_wait_time(nice),
            nice: nice,
            order: None,
        }
    }
    pub fn as_string(&self) -> String {
        format!("Customer #{}", self.id)
    }

    pub fn make_order(&mut self, menu_item: MenuItem) {
        self.order = Some(menu_item);
    }
}

fn calculate_wait_time(nice: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let base_wait = rng.gen_range(5..=10) as u8;
    let nice_adjustment = nice / 2;
    base_wait.saturating_sub(nice_adjustment);
    base_wait as u8
} 

pub fn give_tip() {()}

